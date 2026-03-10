#!/usr/bin/bash

set -euo pipefail

BOLD="\033[1m"
RESET="\033[0m"
RED="\033[1;31m"
YELLOW="\033[1;33m"
GREEN="\033[1;32m"
CYAN="\033[1;36m"
BLUE="\033[1;34m"

info()    { printf "${CYAN}${BOLD}   info${RESET}  %s\n" "$*"; }
success() { printf "${GREEN}${BOLD}     ok${RESET}  %s\n" "$*"; }
warn()    { printf "${YELLOW}${BOLD}   warn${RESET}  %s\n" "$*" >&2; }
error()   { printf "${RED}${BOLD}  error${RESET}  %s\n" "$*" >&2; }
die()     { error "$*"; exit 1; }
ask()     { printf "${BLUE}${BOLD}     ?>  ${RESET}%s " "$*"; }
banner()  { printf "\n${BOLD}${CYAN}  %s${RESET}\n\n" "$*"; }

REPO_URL="https://github.com/aspizu/goboscript"
INSTALL_DIR="${HOME}/.local/share/goboscript"
BIN_DIR="${HOME}/.local/bin"
BINARY_NAME="goboscript"

find_cargo() {
    command -v cargo &>/dev/null          && { command -v cargo; return; }
    [[ -x "${HOME}/.cargo/bin/cargo" ]]   && { printf "%s" "${HOME}/.cargo/bin/cargo"; return; }
    [[ -n "${CARGO_HOME:-}" && -x "${CARGO_HOME}/bin/cargo" ]] \
                                          && { printf "%s" "${CARGO_HOME}/bin/cargo"; return; }
    printf ""
}

install_rustup() {
    info "Downloading rustup installer..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
        || die "rustup installation failed."
    source "${HOME}/.cargo/env" 2>/dev/null || true
    success "Rust toolchain installed."
}

uninstall() {
    banner "Uninstalling goboscript"
    local removed=0

    [[ -f "${BIN_DIR}/${BINARY_NAME}" ]] \
        && { rm -f "${BIN_DIR}/${BINARY_NAME}"; success "Removed ${BIN_DIR}/${BINARY_NAME}"; removed=1; } \
        || warn "Binary not found at ${BIN_DIR}/${BINARY_NAME}"

    [[ -d "${INSTALL_DIR}" ]] \
        && { rm -rf "${INSTALL_DIR}"; success "Removed ${INSTALL_DIR}"; removed=1; } \
        || warn "Source directory not found at ${INSTALL_DIR}"

    [[ $removed -eq 1 ]] && success "goboscript uninstalled." || warn "Nothing to uninstall."
}

install() {
    banner "Installing goboscript"

    CARGO="$(find_cargo)"

    if [[ -z "${CARGO}" ]]; then
        warn "cargo not found."
        ask "Install via rustup? [y/N]"
        read -r response
        [[ "${response}" =~ ^[yY] ]] || die "cargo is required. Aborting."
        install_rustup
        CARGO="$(find_cargo)"
        [[ -z "${CARGO}" ]] && die "cargo still not found. Please restart your shell and re-run."
    fi

    success "Found cargo: ${CARGO}"

    if [[ -d "${INSTALL_DIR}/.git" ]]; then
        info "Updating existing source..."
        git -C "${INSTALL_DIR}" pull --ff-only || warn "git pull failed; using existing source."
    else
        info "Cloning ${REPO_URL}..."
        mkdir -p "$(dirname "${INSTALL_DIR}")"
        git clone --depth 1 "${REPO_URL}" "${INSTALL_DIR}" || die "Failed to clone repository."
    fi

    success "Repository ready."
    info "Building goboscript..."
    mkdir -p "${BIN_DIR}"

    "${CARGO}" install --path "${INSTALL_DIR}" --root "${HOME}/.local" --locked \
        || die "cargo install failed."

    success "Installed to ${BIN_DIR}/${BINARY_NAME}"

    echo ":${PATH}:" | grep -q ":${BIN_DIR}:" || {
        warn "${BIN_DIR} is not in your PATH. Add this to your shell config:"
        printf "\n    ${BOLD}export PATH=\"\$HOME/.local/bin:\$PATH\"${RESET}\n\n"
    }

    success "Done. Run '${BINARY_NAME} --help' to get started."
}

case "${1:-}" in
    --uninstall) uninstall ;;
    "")          install   ;;
    *)           error "Unknown argument: $1"; printf "Usage: %s [--uninstall]\n" "$0"; exit 1 ;;
esac
