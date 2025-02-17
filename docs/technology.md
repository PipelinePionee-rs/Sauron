# Technology Documentation

## Table of Contents
1. [Introduction](#introduction)
2. [Rust](#rust)
3. [Axum](#axum)
4. [Nginx](#nginx)

---

## Introduction
This document is to help outline our reasoning behind each choice made for the technology used in Sauron.
Sauron strive for performance and stability. 

## Rust
Rust is the primary programming language used for the backend of our application. It offers memory safety, high performance, and a strong type system, making it ideal for building stable and performant applications.

## Axum
Axum is our web framework of choice, built specifically for Rust. It is designed for asynchronous, high-performance HTTP applications and integrates seamlessly with the Tokio runtime.

## Nginx
Nginx is used as a reverse proxy for our application. It helps handle incoming requests efficiently and improves the overall performance and security of our services.




