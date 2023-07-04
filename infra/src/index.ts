import * as pulumi from '@pulumi/pulumi';
import * as gcp from "@pulumi/gcp";
import * as docker from '@pulumi/docker';
import {project, imageName, location, memory, cpu, containerPort, concurrency} from './config';

const server = `${location}-docker.pkg.dev`;
const imageTag = `${server}/${project}/${imageName}:latest`;

const authToken = pulumi.output(gcp.organizations.getClientConfig()).accessToken;

const gcrProvider = new docker.Provider('gcr', {
  registryAuth: [{
    address: server,
    username: 'oauth2accesstoken',
    password: authToken,
  }],
});

const image = new docker.RegistryImage(
  'image',
  {name: imageTag, keepRemotely: true},
  {provider: gcrProvider},
);

const apiService = new gcp.projects.Service('services', {
  service: 'run.googleapis.com',
});

// Create a Cloud Run service definition.
const service = new gcp.cloudrun.Service("service", {
  name: 'francis-server',
    location,
    template: {
        spec: {
            containers: [
                {
                    image: pulumi.interpolate`${imageTag}@${image.sha256Digest}`,
                    resources: {
                        limits: {
                            memory,
                            cpu: cpu.toString(),
                        },
                    },
                    ports: [
                        {
                            containerPort,
                        },
                    ],
                }
            ],
            containerConcurrency: concurrency,
        },
    },
}, {dependsOn: [apiService]});

// Create an IAM member to allow the service to be publicly accessible.
const invoker = new gcp.cloudrun.IamMember("invoker", {
    location,
    service: service.name,
    role: "roles/run.invoker",
    member: "allUsers",
});

// Export the URL of the service.
export const url = service.statuses.apply(statuses => statuses[0]?.url);
