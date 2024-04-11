import random

from presentation.dependencies import service
from schemas.plot import LinePlot, _random_names


class TestPlot:
    def test_lineplot_ok(self):
        plot = LinePlot(
            title=".test",
            ylabel="YLABEL",
            xlabel="XLABEL",
            values=[
                (
                    random.choice(_random_names[:4]),
                    random.randint(0, 100),
                    random.randint(0, 10),
                )
                for _ in range(100)
            ],
        )
        print(plot.figsize)

        img = service.draw_lineplot(plot)
        service.save_img(plot, img)
