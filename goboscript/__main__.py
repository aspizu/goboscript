import project
from pathlib import Path
import fmt as Fmt
from typer import Typer

app = Typer()


@app.command()
def build(project_dir: Path, output_pth: Path):
    project.build_gm_project(project_dir).export(output_pth.as_posix())


@app.command()
def fmt(file: Path):
    Fmt.fmt(file)


@app.command()
def fmt_all(pathdir: Path):
    Fmt.fmt_all(pathdir)


if __name__ == "__main__":
    app()
