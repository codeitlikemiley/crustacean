# Load .env if it exists
-include .env

# GCP Configuration (Defaults if not set in .env)
PROJECT_ID   ?= YOUR_PROJECT_ID
REGION       ?= us-central1
SERVICE_NAME ?= tuts
GCP_ACCOUNT  ?= your-email@example.com
IMAGE_NAME   := gcr.io/$(PROJECT_ID)/$(SERVICE_NAME)

.PHONY: all build run push deploy auth-login setup-project gcp-context ship ci-deploy

all: build

# --- Internal: Ensure correct GCP context ---
gcp-context:
	@echo "Switching to GCP Account: $(GCP_ACCOUNT) and Project: $(PROJECT_ID)..."
	@gcloud config set account $(GCP_ACCOUNT)
	@gcloud config set project $(PROJECT_ID)

# --- Local Development ---

build:
	docker build -t $(SERVICE_NAME) .

run:
	docker run -p 8080:80 $(SERVICE_NAME)

# --- GCP Deployment ---

auth-login: gcp-context
	gcloud auth login $(GCP_ACCOUNT)

setup-project: gcp-context
	gcloud services enable run.googleapis.com containerregistry.googleapis.com

push: gcp-context
	gcloud builds submit --tag $(IMAGE_NAME) .

deploy: gcp-context
	gcloud run deploy $(SERVICE_NAME) \
		--image $(IMAGE_NAME) \
		--platform managed \
		--region $(REGION) \
		--allow-unauthenticated

# Combined command for quick deployment
ship: push deploy

# --- Utility ---

# Get the deployed URL
url: gcp-context
	@gcloud run services describe $(SERVICE_NAME) \
		--platform managed \
		--region $(REGION) \
		--format 'value(status.url)'

# Open the deployed URL in the browser
open:
	@open $$(gcloud run services describe $(SERVICE_NAME) --platform managed --region $(REGION) --format 'value(status.url)')

# --- GitHub Workflow Support ---

ci-deploy:
	gcloud run deploy $(SERVICE_NAME) \
		--image $(IMAGE_NAME) \
		--platform managed \
		--region $(REGION) \
		--allow-unauthenticated

