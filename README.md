# Artificial_Intelligence(NLP, Natural Language Processing models and pipelines.)

- rust-bert[![crates.io](https://img.shields.io/crates/v/rust-bert.svg)](https://crates.io/crates/rust-bert)![Crates.io](https://img.shields.io/crates/l/rust-bert)![druidDownloads](https://img.shields.io/crates/d/rust-bert.svg)<a href="https://github.com/guillaume-be/rust-bert"><img alt="githubicon" width="20px" src="https://user-images.githubusercontent.com/67513038/218287708-001511d7-1cce-42d3-92d2-4a61193b38f0.png" /></a>
  ![star](https://img.shields.io/github/stars/guillaume-be/rust-bert.svg)

  - Rust native ready-to-use NLP pipelines and transformer-based models (BERT, DistilBERT, GPT2,...)

  - https://github.com/guillaume-be/rust-bert

  - https://crates.io/crates/rust-bert

# Rust MachineLearning<img alt="rustmascot" width="26px" src="https://user-images.githubusercontent.com/67513038/213403213-1b1b3efc-ce53-4825-9dfc-e9bf2956a7f4.svg" /></a>

- linfa[![crates.io](https://img.shields.io/crates/v/linfa.svg)](https://crates.io/crates/linfa)![Crates.io](https://img.shields.io/crates/l/linfa)![druidDownloads](https://img.shields.io/crates/d/linfa.svg)<a href="https://github.com/guillaume-be/rust-bert"><img alt="githubicon" width="20px" src="https://user-images.githubusercontent.com/67513038/218287708-001511d7-1cce-42d3-92d2-4a61193b38f0.png" /></a>
  ![star](https://img.shields.io/github/stars/rust-ml/linfa.svg)

  - A Rust machine learning framework.

  - https://github.com/rust-ml/linfa

  - https://crates.io/crates/linfa

- ndarray[![crates.io](https://img.shields.io/crates/v/ndarray.svg)](https://crates.io/crates/ndarray)![Crates.io](https://img.shields.io/crates/l/ndarray)![druidDownloads](https://img.shields.io/crates/d/ndarray.svg)<a href="https://github.com/guillaume-be/rust-bert"><img alt="githubicon" width="20px" src="https://user-images.githubusercontent.com/67513038/218287708-001511d7-1cce-42d3-92d2-4a61193b38f0.png" /></a>
  ![star](https://img.shields.io/github/stars/rust-ndarray/ndarray.svg)

  - ndarray: an N-dimensional array with array views, multidimensional slicing, and efficient operations

  - https://github.com/rust-ndarray/ndarray

  - https://crates.io/crates/ndarray

<br>

<hr>

# brew install libtorch(macOS)

- pytorch ????????? ?????? ?????? ????????? ??? !!!

```
export LIBTORCH='/opt/homebrew/Cellar/pytorch/1.13.1'

export LD_LIBRARY_PATH=$LIBTORCH:$LD_LIBRARY_PATH


echo $LD_LIBRARY_PATH
/opt/homebrew/Cellar/pytorch/1.13.1:
```

# Rust Artificial Intelligence (The Simple Way)

https://youtu.be/StMP7g-0wK4

https://github.com/guillaume-be/rust-bert

<br>

# The AI community building the future.

https://huggingface.co/

<br>

<hr>

# How to Build a Machine Learning Model in Rust

https://www.freecodecamp.org/news/how-to-build-a-machine-learning-model-in-rust/

# Rust Machine Learning Book

https://rust-ml.github.io/book/chapter_1.html

<br>

# Unicode (Vim Plug-in)

https://github.com/chrisbra/unicode.vim

- Ex commands:

```
:UnicodeTable    - Print Unicode Table in new window
:Digraphs        - Search for specific digraph char
:UnicodeSearch   - Search for specific unicode char
:UnicodeSearch!  - Search for specific unicode char (and add at current cursor position)
:UnicodeName     - Identify character under cursor (like ga command)
:DownloadUnicode - Download (or update) Unicode data
:UnicodeCache    - Create cache file

```

<br>

- Normal mode commands:

```

<C-X><C-G>  - Complete Digraph char
<C-X><C-Z>  - Complete Unicode char
<F4>        - Combine characters into digraphs
Scripting Functions:
unicode#FindUnicodeBy() - Find unicode characters
unicode#FindDigraphBy() - Find Digraph char
unicode#Digraph()       - Returns digraph char
unicode#UnicodeName()   - Identifies unicode character (by value)
```

<hr>

# Natural Language Processing for Rust

https://github.com/lexi-sh/rs-natural

<br>

<hr>

# This repository is a list of machine learning libraries written in Rust. It's a compilation of GitHub repositories, blogs, books, movies, discussions, papers, etc. ????

https://github.com/vaaaaanquish/Awesome-Rust-MachineLearning

<br>

<hr>

# Here is an example of a simple machine learning program written in the Rust programming language:

- GPT3

https://chat.openai.com/chat

This program sets up a game context and creates a new MainState struct to hold the game's state. It then runs the game loop, which handles events and updates the game state accordingly. The MainState struct and its associated methods and the event handling methods can be customized to implement the specific game logic and mechanics.

```

use ndarray::{Array, Array2};
use rand::{thread_rng, Rng};
use rusty_machine::linalg::{Matrix, Vector};
use rusty_machine::learning::k_means::KMeansClassifier;
use rusty_machine::learning::UnSupModel;

// Create some random data to cluster
let mut rng = thread_rng();
let data: Vec<Vector<f64>> = (0..100)
    .map(|_| Vector::new(vec![rng.gen(), rng.gen()]))
    .collect();
let data = Matrix::new(100, 2, data);

// Create a new k-means model with 2 clusters
let mut model = KMeansClassifier::new(2);

// Train the model on the data
model.train(&data)?;

// Predict the cluster assignments for the data
let assignments = model.predict(&data)?;

// Print the cluster assignments
println!("Cluster assignments: {:?}", assignments);
This program creates some random data and uses the KMeansClassifier type from the rusty_machine crate to train a k-means clustering model on the data. It then uses the trained model to predict the cluster assignments for the data and prints the results. The KMeansClassifier type and the train and predict methods can be customized to implement different machine learning algorithms and apply them to different types of data.




```

- ????????? ?????? ??? ?????????. ???

https://athemathmo.github.io/rusty-machine/doc/rusty_machine/index.html

<br>

<hr>

# How to Build a Machine Learning Model in Rust

https://www.freecodecamp.org/news/how-to-build-a-machine-learning-model-in-rust/

<br>
# Machine_Learning_Rust
