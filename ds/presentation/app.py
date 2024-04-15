import time

from fastapi import FastAPI, Request
from presentation import anime_router, plot_router

app = FastAPI(title="Fun Data Science!!!")
app.include_router(anime_router.router)
app.include_router(plot_router.router)


@app.middleware("http")
async def add_process_time_header(request: Request, call_next):
    t0 = time.monotonic()
    response = await call_next(request)
    process_time = time.monotonic() - t0
    response.headers["X-Process-Time"] = str(process_time)
    return response


@app.get("/health")
def health() -> dict[str, bool]:
    return {"success": True}
