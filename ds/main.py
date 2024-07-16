from logging import _nameToLevel

import uvicorn
from loguru import logger
from presentation.app import app as uvicorn_app
from shared.settings import app_settings

if __name__ == "__main__":
    logger.info(
        "starting server on {}:{} with {} workers",
        app_settings.uvicorn.host,
        app_settings.uvicorn.port,
        app_settings.uvicorn.workers,
    )

    uvicorn.run(
        uvicorn_app,
        host=app_settings.uvicorn.host,
        port=app_settings.uvicorn.port,
        workers=app_settings.uvicorn.workers,
        log_level=_nameToLevel[app_settings.uvicorn.log_level],
    )
