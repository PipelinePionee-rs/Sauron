# How We Are DevOps: Sauron Search Engine Project

In our search engine project, we've embraced DevOps principles throughout our development process, transforming the whoknows_variations searchengine codebase into a modern Rust implementation while maintaining our JavaScript/HTML/CSS frontend.

We've chosen to frame our "How are you DevOps?" assignment through the CALMS framework because it provides a comprehensive lens that goes beyond technical practices alone. While tooling and automation are important aspects of DevOps, we recognize that successful DevOps implementation fundamentally requires cultural and organizational transformation.
However, we are not DevOps - But we have stayed committed to the elective and its learning goals - and endeavoured on a journey of continuous learning. 

## Culture, Automation, Lean, Measurement, and Sharing (CALMS)

*Culture*
Our team has cultivated DevOps culture based on psychological safety. We encourage open communication, where team members feel comfortable sharing ideas, raising concerns, and making mistakes. This has fostered an environment of continuous learning and innovation, where we support and help each other rather than assigning blame. Our regular meetings allow us to reflect on our processes and implement improvements collaboratively. 

*Automation*
We've implemented a robust CI/CD pipeline using GitHub Actions, which has significantly improved our development workflow. Our automated workflows run tests on every pull request and on code pushes to the dev and main branches, ensuring we catch issues early in the development cycle. After successful builds, our system automatically deploys to our production environment, reducing manual intervention and potential human error.

*Lean*
We've adopted lean principles by focusing on delivering value continuously in small increments. By working in short iterations and getting rapid feedback, we minimize waste and optimize our workflow. Our migration from Python to Rust demonstrates our commitment to eliminating technical debt and building more efficient systems that deliver better performance for our users.

*Measurement*
We've implemented structured logging in our Rust backend to improve our operational visibility. Our website integrates with a simulation provided by our lecturer, which allows us to view logs and monitor system behavior, and act on potential issues fast. We also use Postman for health checks and track KPIs on the server, giving us insights into uptime and performance. These tools help us detect issues early, validate our changes, and make more informed decisions during development.

*Sharing*
Knowledge sharing is a core part of our process. While we haven’t practiced pair programming, we hold regular online meetings where team members actively support each other by discussing challenges, sharing insights, and offering help. These online meetings are held on discord, on a channel exclusively for our group in the DevOps elective. This channel is a lively space, where teammembers share struggles and victories - and mostly, a safe space where we support each other through challenges and celebrate successes. 
Although our README and documentation are still evolving, they provide a starting point for understanding the project and contributing to the codebase. 
These collaborative practices have fostered open communication and strengthened the connection between team members, supporting a culture of continuous learning.

##Improvements
Throughout our DevOps journey, we've focused on applying key principles week by week to enhance our development process and team dynamics. Each week, we've concentrated on a specific aspect of DevOps. These weekly principles have guided our practices and helped us build a more efficient, collaborative, and resilient development environment.

However, there are still areas where we can improve. Communication within the team needs enhancement; team members should increase their openness and the frequency of check-ins to ensure everyone is aligned and informed. Implementing pair programming or more frequent progress sharing can help bridge knowledge gaps and foster a more collaborative environment, especially given the diverse levels of experience within the team.

By addressing these areas, we can further strengthen our DevOps culture, improve our overall productivity, and ensure that our development process remains agile and effective.