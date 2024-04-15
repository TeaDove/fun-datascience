from fastapi import APIRouter
from presentation.dependencies import container
from schemas.plot import Bar, Graph, LinePlot, Points, TimeSeries
from starlette.responses import StreamingResponse

router = APIRouter(prefix="/plot")


@router.post("/points", response_class=StreamingResponse)
def draw_fig(input_: Points) -> StreamingResponse:
    return StreamingResponse(
        container.plot_service.draw_points(input_),
        media_type=f"image/{input_.image_format}",
    )


@router.post("/histogram", response_class=StreamingResponse)
def draw_bar(input_: Bar) -> StreamingResponse:
    return StreamingResponse(
        container.plot_service.draw_bar(input_),
        media_type=f"image/{input_.image_format}",
    )


@router.post("/lineplot", response_class=StreamingResponse)
def draw_lineplot(input_: LinePlot) -> StreamingResponse:
    return StreamingResponse(
        container.plot_service.draw_lineplot(input_),
        media_type=f"image/{input_.image_format}",
    )


@router.post("/timeseries", response_class=StreamingResponse)
def draw_timeseries(input_: TimeSeries) -> StreamingResponse:
    return StreamingResponse(
        container.plot_service.draw_timeseries(input_),
        media_type=f"image/{input_.image_format}",
    )


@router.post("/graph", response_class=StreamingResponse)
def draw_graph(input_: Graph) -> StreamingResponse:
    return StreamingResponse(
        container.plot_service.draw_graph(input_),
        media_type=f"image/{input_.image_format}",
    )


@router.post("/graph-as-heatmap", response_class=StreamingResponse)
def draw_graph_as_heatmap(input_: Graph) -> StreamingResponse:
    return StreamingResponse(
        container.plot_service.draw_graph_as_heatmap(input_),
        media_type=f"image/{input_.image_format}",
    )
