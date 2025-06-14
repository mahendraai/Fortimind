from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from inference import perform_inference

app = FastAPI()

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

@app.get("/")
def read_root():
    return {"message": "Welcome to the AI Server"}

@app.post("/inference/")
def get_inference(data: dict):
    result = perform_inference(data)
    return {"result": result}