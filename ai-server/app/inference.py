from fastapi import FastAPI
from pydantic import BaseModel
import joblib
import numpy as np

app = FastAPI()

# Load the trained models
malware_classifier = joblib.load("models/malware_classifier.onnx")
ransomware_lstm = joblib.load("models/ransomware_lstm.onnx")

class InferenceRequest(BaseModel):
    features: list

@app.post("/infer/malware")
async def infer_malware(request: InferenceRequest):
    features = np.array(request.features).reshape(1, -1)
    prediction = malware_classifier.predict(features)
    return {"prediction": prediction.tolist()}

@app.post("/infer/ransomware")
async def infer_ransomware(request: InferenceRequest):
    features = np.array(request.features).reshape(1, -1)
    prediction = ransomware_lstm.predict(features)
    return {"prediction": prediction.tolist()}