import io

import torch
import torch.nn as nn
import torchvision.models as models
import torchvision.transforms as T
from loguru import logger
from PIL import Image


class NotAnImage(Exception):
    ...


class AnimeService:
    __transformations = T.Compose([T.ToTensor(), T.Normalize((0, 0, 0), (1, 1, 1))])

    def __init__(self, weights_files: str):
        """
        model_path - location of model
        """
        self.__device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
        self.__model = models.resnet50(weights=True)
        self.__model.fc = nn.Sequential(nn.Linear(2048, 1, bias=True), nn.Sigmoid())
        self.__model.load_state_dict(
            torch.load(weights_files, map_location=self.__device)
        )
        self.__model.eval()

        logger.info("anime.service.initialized")

    def make_prediction(self, img_bytes: bytes) -> float:
        """
        img_path is path of image, lol
        """
        try:
            img = Image.open(io.BytesIO(img_bytes))
        except Exception as exc:
            raise NotAnImage("passed image is not and image") from exc
        img = img.convert("RGB")
        if img.size != (224, 224):
            img = img.resize((224, 224))

        img_tensor = self.__transformations(img)
        return float(
            self.__model(torch.reshape(img_tensor, (1, 3, 224, 224)).to(self.__device))[
                0
            ][0]
            .detach()
            .numpy()
        )
