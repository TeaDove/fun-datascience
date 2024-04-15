from dataclasses import dataclass

from service.anime_service import AnimeService
from service.plot_service import PlotService


@dataclass
class Container:
    plot_service: PlotService
    anime_service: AnimeService


def init_combat_container() -> Container:
    return Container(
        plot_service=PlotService(), anime_service=AnimeService("weights.pth")
    )
