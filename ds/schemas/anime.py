from pydantic import BaseModel, Field


class AnimePredictionResponse(BaseModel):
    prediction: float = Field(..., example=0.7)
