TAG = beta

.PHONY: docker_deploy
docker_deploy:
	cd .. && docker buildx build --platform linux/amd64 -t yourorg/email-validator-sqs:$(TAG) -f sqs/Dockerfile .
