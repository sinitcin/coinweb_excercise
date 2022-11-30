# Task 01: Package Manager

Solve the problem described below, and submit a link to the repository containing your solution to us.

## Statement

Suppose that you are working on  package manager. 
Your task is to implement the retrieval of binary packages in order for them to be installed in the system.
The package manager is capable of: given the name of a package, list of all repositories where the package can be found.
Assume that the connection to the repositories is not stable and it can fail at any moment. 
Hence, instead of downloading the package from a single repository, you will download the package from all sources in parallel, and if one of the downloads fails, you still have the other downloads.
When a download finishes, you must stop the rest of downloads, and install the package. If all downloads fail, you must inform the user about this fact.

### Bonus

- If a download fails, retry the download.

### Notes

The goal of this exercise is to evaluate your knowledge of async Rust. 

Please, avoid using utilities like [tokio::JoinSet](https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html), nor crates that resolves this task for you.

## Duration

Up to 4 hours.
