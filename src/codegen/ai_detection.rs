// https://github.com/vercel/vercel/blob/main/packages/detect-agent/src/index.ts
// https://github.com/sdairs/is-ai-agent/blob/main/src/lib.rs
const AGENT_NONEMPTY_ENV_VARS: &[&str] = &[
    "AMP_CURRENT_THREAD_ID",
    "ANTIGRAVITY_AGENT",
    "ANTIGRAVITY_PROJECT_ID",
    "AUGMENT_AGENT",
    "CLAUDECODE",
    "CLAUDE_CODE",
    "CLAUDE_CODE_ENTRYPOINT",
    "CLAUDE_CODE_EXECPATH",
    "CLAUDE_CODE_IS_COWORK",
    "CLAUDE_CODE_SESSION_ID",
    "CLINE_ACTIVE",
    "CLINE_TASK_ID",
    "CODEBUDDY",
    "CODEBUDDY_PROJECT_DIR",
    "CODEBUDDY_SESSION_ID",
    "CODEX_CI",
    "CODEX_ROOT",
    "CODEX_SANDBOX",
    "CODEX_SANDBOX_NETWORK_DISABLED",
    "CODEX_THREAD_ID",
    "COPILOT_AGENT",
    "COPILOT_AGENT_JOB_ID",
    "COPILOT_AGENT_SESSION_ID",
    "COPILOT_ALLOW_ALL",
    "COPILOT_CLI",
    "COPILOT_GITHUB_TOKEN",
    "COPILOT_MODEL",
    "CRUSH",
    "CURSOR_AGENT",
    "CURSOR_SANDBOX",
    "CURSOR_TRACE_ID",
    "FIREBENDER_TERMINAL",
    "GEMINI_CLI",
    "GOOSE_TERMINAL",
    "IFLOW_CLI",
    "KIRO_AGENT_PATH",
    "OPENCODE",
    "OPENCODE_APP_INFO",
    "OPENCODE_BIN_PATH",
    "OPENCODE_CLIENT",
    "OPENCODE_MODES",
    "OPENCODE_PID",
    "OPENCODE_SERVER",
    "OZ_RUN_ID",
    "PI_CODING_AGENT",
    "QWEN_CODE",
    "REPL_ID",
    "ROO_CODE_TASK_ID",
    "TRAE_AI_SHELL_ID",
    "VECLI_DIR",
    "VECLI_SANDBOX",
];

const AGENT_NONBLANK_ENV_VARS: &[&str] = &[
    "CODEX_SESSION_ID",
    "GROK_SESSION_ID",
    "HERMES_SESSION_ID",
    "JUNIE_SHIM_PATH",
    "PI_SESSION_ID",
];

const AGENT_EXACT_ENV_VALUES: &[(&str, &str)] = &[
    ("CLAUDE_CODE_CHILD_SESSION", "1"),
    ("DSH_SHELL", "1"),
    ("HERMES_AGENT", "true"),
    ("KILO", "1"),
    ("OPENCLAW_SHELL", "exec"),
    ("VTCODE", "1"),
];

fn env_is_nonempty(names: &[&str], env: &impl Fn(&str) -> Option<String>) -> bool {
    names
        .iter()
        .any(|name| env(name).is_some_and(|value| !value.is_empty()))
}

fn env_is_nonblank(names: &[&str], env: &impl Fn(&str) -> Option<String>) -> bool {
    names
        .iter()
        .any(|name| env(name).is_some_and(|value| !value.trim().is_empty()))
}

fn env_equals(values: &[(&str, &str)], env: &impl Fn(&str) -> Option<String>) -> bool {
    values
        .iter()
        .any(|(name, expected)| env(name).is_some_and(|value| value == *expected))
}

fn env_trimmed_equals(name: &str, expected: &str, env: &impl Fn(&str) -> Option<String>) -> bool {
    env(name).is_some_and(|value| value.trim() == expected)
}

fn env_contains(names: &[&str], marker: &str, env: &impl Fn(&str) -> Option<String>) -> bool {
    names
        .iter()
        .any(|name| env(name).is_some_and(|value| value.contains(marker)))
}

fn env_is_enabled(names: &[&str], env: &impl Fn(&str) -> Option<String>) -> bool {
    names.iter().any(|name| {
        env(name).is_some_and(|value| {
            let value = value.trim();
            !value.is_empty()
                && !["0", "false", "no", "off"]
                    .iter()
                    .any(|disabled| value.eq_ignore_ascii_case(disabled))
        })
    })
}

pub(super) fn is_run_by_agent(env: impl Fn(&str) -> Option<String>) -> bool {
    env_is_nonempty(AGENT_NONEMPTY_ENV_VARS, &env)
        || env_is_nonblank(AGENT_NONBLANK_ENV_VARS, &env)
        || env_equals(AGENT_EXACT_ENV_VALUES, &env)
        || env_is_enabled(&["AGENT", "AI_AGENT"], &env)
        || env_trimmed_equals("CURSOR_EXTENSION_HOST_ROLE", "agent-exec", &env)
        || env_contains(&["AWS_EXECUTION_ENV"], "AmazonQ-For-CLI", &env)
        || (env_is_nonempty(&["AGENT_CONTEXT_OUT"], &env)
            && env_is_nonempty(&["AGENT_DISPLAY_OUT"], &env))
        || env_contains(&["PS1", "PROMPT_COMMAND"], "###PS1JSON###", &env)
}
