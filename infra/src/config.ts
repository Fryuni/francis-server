import * as pulumi from '@pulumi/pulumi';
import * as gcp from '@pulumi/gcp';

// Import the program's configuration settings.
const config = new pulumi.Config();
export const imageName = config.require("imageName");
export const appPath = config.require("appPath");
export const containerPort = config.getNumber("containerPort") ?? 8080;
export const cpu = config.getNumber("cpu") ?? 1;
export const memory = config.get("memory") ?? "1Gi";
export const concurrency = config.getNumber("concurrency") ?? 80;

export const location = gcp.config.region!;
export const project = gcp.config.project;

