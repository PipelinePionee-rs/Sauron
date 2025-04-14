### Speeding up CI workflow
*11-4-25*

We have noticed that our CI/CD workflow takes a while to run, causing frustration and 
periods where we're stuck sitting doing nothing waiting for workflows to run before we can continue our work.

Seeking a solution to this, we've found blacksmith.sh, an alternative to using Github's runners. These run at twice the speed for half the cost, effectively cutting our deployment time in half.

Furthermore, we've implemented caching on these new runners, enabling us to speed up the process even further.
The speed of our workflows has improved as such:

Build/Test: 3 minutes           -> 15 seconds

Smoke Test: 7 minutes           -> 1 minute 30 seconds

Linting:    1 minute 30 seconds -> 1 minute

CD:         8 minues            -> 5 minutes 30 seconds