import multiprocessing as mp

from pydantic import BaseModel
from pydantic_settings import BaseSettings, SettingsConfigDict


class UvicornSettings(BaseModel):
    host: str = "localhost"
    port: int = 8000
    workers: int = mp.cpu_count() * 2
    log_level: str = "WARNING"


class AppSettings(BaseSettings):
    uvicorn: UvicornSettings = UvicornSettings()

    model_config = SettingsConfigDict(
        env_prefix="_", env_file=".env", env_nested_delimiter="__"
    )


app_settings = AppSettings()
