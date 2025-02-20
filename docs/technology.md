# Technology Documentation

## Table of Contents
1. [Introduction](#introduction)
2. [Rust](#rust)
3. [Axum](#axum)
4. [Nginx](#nginx)
5. [Github](#github)

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

## Github / Projects / Actions

### Github
We use Github for version control and easy collaboration, making sure to implement branching strategies to keep our project structured and clean. 

### Projects 
helps us keep track of current issues and features, and prioritize the critical functions of our application. Keeping track of issues, and who is assigned to a given task, makes sure we don't interfere with each others work.

### Actions
Actions currently automates our building and testing, making sure code that gets pushed to the repo is functional, and actually accomplishes its goal.

Later, we will also use actions for Continous Deployment using Docker.



