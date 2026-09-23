from logging import INFO, Logger, basicConfig, getLogger

basicConfig(
    level=INFO,
    format="%(asctime)s [%(levelname)s] %(name)s: %(message)s",
    datefmt="%Y-%m-%d %H:%M:%S",
)


def get_logger(name: str) -> Logger:
    logger: Logger = getLogger(name=name)
    return logger
