import enum
import uuid
from datetime import datetime

from numpy import random
from pydantic import BaseModel, Field, field_validator
from typing_extensions import Annotated

colors = ("RED", "BLUE", "GREEN")
_random_names = [
    "maggot",
    "varam",
    "kammus",
    "sarayn",
    "enarvyne",
    "alammu",
    "irethi",
    "neldam",
    "dren",
    "anasour",
    "irarvy",
    "vandal",
    "tadaves",
    "seran",
    "llaalam",
    "worker",
    "dalamus",
    "vandal",
    "gidave",
    "sendal",
    "othralen",
    "tedril",
    "girothran",
    "ararvy",
    "maryon",
    "llaala",
    "faveran",
    "gadaves",
    "uradasou",
    "berendal",
    "maggot",
    "heloth",
    "neldammu",
    "othren",
    "midaves",
    "deras",
    "vedaves",
    "ienevala",
]


class Plot(BaseModel):
    title: str | None = None
    ylabel: str | None = None
    xlabel: str | None = None

    x_figsize: float = 20
    figsize: Annotated[tuple[float, float] | None, Field(validate_default=True)] = None

    image_format: str = "jpeg"
    label_font_size: int = 24

    @field_validator("figsize", mode="before")
    @classmethod
    def set_figsize(cls, v: tuple[float, float] | None, values) -> tuple[float, float]:
        if v is not None:
            return v

        return values.data["x_figsize"], values.data["x_figsize"] * 9 / 16


class Point(BaseModel):
    id_: uuid.UUID = Field(default_factory=uuid.uuid4, alias="id")
    color: str

    lat: float
    lon: float


class Points(Plot):
    points: list[Point] = Field(
        example=[
            Point(
                color=random.choice(colors),
                lat=random.rand() * 20 + 40,
                lon=random.rand() * 20 + 40,
            )
            for _ in range(random.randint(10, 20))
        ]
    )


class Bar(Plot):
    values: dict[str, float] = Field(
        example={
            random.choice(_random_names): random.randint(0, 100)
            for _ in range(random.randint(10, 20))
        }
    )

    limit: int | None = Field(None, example=None)
    asc: bool = True


class TimeSeries(Plot):
    values: dict[str, dict[datetime, float]] = Field(
        example={
            random.choice(_random_names): {
                datetime(
                    2024, random.randint(1, 12), random.randint(1, 28)
                ): random.randint(0, 100)
                for _ in range(20)
            }
            for _ in range(3)
        }
    )

    only_time: bool = False


class LinePlot(Plot):
    values: list[tuple[str, float, float]] = Field(
        example=[
            (
                random.choice(_random_names[:4]),
                random.randint(0, 100),
                random.randint(0, 10),
            )
            for _ in range(100)
        ]
    )


class GraphEdge(BaseModel):
    first: str
    second: str
    weight: float


class GraphNode(BaseModel):
    image: bytes | None = None
    weight: float = 1


class GraphLayout(str, enum.Enum):
    SPRING = "spring"
    CIRCULAR = "circular"
    SPECTRAL = "spectral"
    CIRCULAR_TREE = "circular_tree"
    NEATO = "neato"


class Graph(Plot):
    edges: list[GraphEdge] = Field(
        example=[
            GraphEdge(
                first=random.choice(_random_names),
                second=random.choice(_random_names),
                weight=random.randint(0, 100),
            )
            for _ in range(30)
        ]
    )
    weighted_edges: bool = True
    layout: GraphLayout = GraphLayout.CIRCULAR
    nodes: dict[str, GraphNode] | None = None
    root_node: str | None = None
