from fastapi import APIRouter, HTTPException, UploadFile
from presentation.dependencies import container
from schemas.anime import AnimePredictionResponse
from service.anime_service import NotAnImage

router = APIRouter(prefix="/anime")


@router.post("/predict", response_model=AnimePredictionResponse)
async def amine_predict(image: UploadFile) -> AnimePredictionResponse:
    try:
        return AnimePredictionResponse(
            prediction=container.anime_service.make_prediction(await image.read())
        )
    except NotAnImage:
        raise HTTPException(status_code=422, detail="inputed file is not an image")
