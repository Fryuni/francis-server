import * as pulumi from '@pulumi/pulumi';
import * as gcp from "@pulumi/gcp";
import {project, imageName, location, memory, cpu, containerPort, concurrency} from './config';

const imageTag = `gcr.io/${project}/${imageName}:latest`;

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
                    image: imageTag,
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
