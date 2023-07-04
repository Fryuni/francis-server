deploy:
  docker buildx build . --load -t southamerica-east1-docker.pkg.dev/lferraz-personal-servers-1/images/francis-server:latest
  pulumi up -y --cwd infra
