from fastapi import APIRouter, UploadFile
from presentation.dependencies import container
from schemas.anime import AnimePredictionResponse

router = APIRouter(prefix="/anime")


@router.post("/predict", response_model=AnimePredictionResponse)
async def amine_predict(image: UploadFile) -> AnimePredictionResponse:
    return AnimePredictionResponse(
        prediction=container.anime_service.make_prediction(await image.read())
    )
