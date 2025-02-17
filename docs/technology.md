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
Rust is used for the backend, It offers memory safety, high performance, and a strong type system, making it ideal for stable and performant applications.

## Axum
Axum is our web framework of choice, built specifically for Rust. It is designed for asynchronous, high-performance HTTP applications and integrates seamlessly with the Tokio runtime.
There were other great options like Actix, but the choice eventually fell on Axum due to benchmarks from [TechEmpower](https://www.techempower.com/benchmarks/#hw=ph&test=fortune&section=data-r22)
![Image showing axum rank 6 on a web framework benchmark](./images/Axum%20Benchmark.png)



## Nginx
Nginx is used as a reverse proxy for our backend API, and to serve our static html and javascript files. 
Since stability and performance is our goal, Nginx is the obvious choice.





