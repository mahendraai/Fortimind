# Sandbox Design Document

## Overview
The sandboxing feature is a critical component of the EDR (Endpoint Detection and Response) agent, designed to isolate and control the execution of monitored processes. This document outlines the design principles, architecture, and implementation details of the sandboxing functionality for both Linux and Windows environments.

## Objectives
- To provide a secure environment for executing potentially harmful processes.
- To monitor and control the behavior of processes within the sandbox.
- To prevent unauthorized access to system resources from monitored processes.

## Architecture
The sandboxing architecture consists of two main components:
1. **Linux Sandbox**: Implemented in `agent/src/sandbox/linux.rs`, this component leverages Linux namespaces and cgroups to create isolated environments for processes.
2. **Windows Sandbox**: Implemented in `agent/src/sandbox/windows.rs`, this component utilizes Windows containers and security features to achieve process isolation.

### Linux Sandbox
- **Namespaces**: Utilize user, PID, and network namespaces to isolate process resources.
- **Cgroups**: Control resource allocation (CPU, memory) for processes running in the sandbox.
- **Monitoring**: Implement hooks to monitor system calls and file access within the sandbox.

### Windows Sandbox
- **Containers**: Use Windows containers to create isolated environments for executing processes.
- **Security Policies**: Define security policies to restrict access to system resources.
- **Monitoring**: Implement logging and monitoring of process activities within the sandbox.

## Implementation Details
- **Process Creation**: Both sandboxes will intercept process creation requests and spawn processes within their respective isolated environments.
- **Resource Management**: Implement resource limits to prevent sandboxes from consuming excessive system resources.
- **Communication**: Establish secure communication channels between the agent and the sandboxed processes for telemetry and control.

## Conclusion
The sandboxing feature is essential for enhancing the security posture of the EDR agent. By isolating potentially harmful processes, we can mitigate risks and improve the overall effectiveness of threat detection and response capabilities. Further testing and validation will be conducted to ensure the robustness and reliability of the sandbox implementations.