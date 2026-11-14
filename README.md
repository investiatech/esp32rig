# ESP32Rig Rust IoT GenAI

**A Rust framework for building AI agents on ESP32 microcontrollers.**

ESP32Rig is a framework for developing lightweight AI agents that run directly on ESP32 devices and communicate with
remote Large Language Models (LLMs).

The goal is to provide a simple, modular and extensible API for building embedded AI agents.

## Goals

- Rust-native API.
- `no_std` support.
- Lightweight agent runtime.
- Local tool registration and execution.
- Remote LLM integration.
- Modular and extensible architecture.
- ESP32 hardware support.

## Target Hardware

Initial development target:

- M5Stack NanoC6
- ESP32-C6
- Rust and ESP-HAL

## Model Providers

ESP32Rig is designed to support multiple LLM providers through a modular provider adapter architecture.
The goal is to provide a unified interface for communicating with different language models while keeping the core
framework independent of provider-specific APIs.

## Project Architecture

ESP32Rig is designed as a modular Rust framework consisting of an independent core and separate provider integrations.

Planned crate structure:

- `esp32rig-core` – independent framework core.
- `esp32rig-provider-ollama` – Ollama integration.
- `esp32rig-provider-llamacpp` – llama.cpp integration.
- `esp32rig-provider-openai` – OpenAI integration.
- `esp32rig-provider-anthropic` – Anthropic integration.
- `esp32rig-provider-gemini` – Google Gemini integration.

The core will remain independent of specific LLM providers. Each provider will implement a common model interface,
allowing applications to switch between providers without changing their agent logic.

Provider adapters will handle provider-specific:

- API communication.
- Request serialization.
- Response deserialization.
- Tool calling.
- Authentication and configuration.
- Error handling.

The provider architecture is designed to allow new integrations without modifying the ESP32Rig core.

## Project Status

Early development. Architecture and core API are being designed.

## License

The project will probably be released under the MIT License. Final licensing terms are yet to be determined.