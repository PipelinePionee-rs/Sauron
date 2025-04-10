# How We Are DevOps: Sauron Search Engine Project

In our search engine project, we've embraced DevOps principles throughout our development process, transforming the whoknows_variations searchengine codebase into a modern Rust implementation while maintaining our JavaScript/HTML/CSS frontend.

We've chosen to frame our "How are you DevOps?" assignment through the CALMS framework because it provides a comprehensive lens that goes beyond technical practices alone. While tooling and automation are important aspects of DevOps, we recognize that successful DevOps implementation fundamentally requires cultural and organizational transformation.

## Culture, Automation, Lean, Measurement, and Sharing (CALMS)

*Culture*
Our team has cultivated a strong DevOps culture based on psychological safety. We encourage open communication, where team members feel comfortable sharing ideas, raising concerns, and making mistakes. This has fostered an environment of continuous learning and innovation, where we support and help each other rather than assigning blame. Our regular meetings allow us to reflect on our processes and implement improvements collaboratively.

*Automation*
We've implemented a robust CI/CD pipeline using GitHub Actions, which has significantly improved our development workflow. Our automated workflows run tests on every pull request and code push from dev to main branch, ensuring we catch issues early in the development cycle. After successful builds, our system automatically deploys to our production environment, reducing manual intervention and potential human error.

*Lean*
We've adopted lean principles by focusing on delivering value continuously in small increments. By working in short iterations and getting rapid feedback, we minimize waste and optimize our workflow. Our migration from Python to Rust demonstrates our commitment to eliminating technical debt and building more efficient systems that deliver better performance for our users.
