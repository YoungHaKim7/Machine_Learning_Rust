# link

- 머신러닝 Tutorial
  - [머신러닝기초](#머신러닝기초)
    - [여기에 정리중..](./004_Machine_Math_Basic_matrix_cal)

- [초보자를 위한 Vector Embeddings 가이드 (timescale.com)](#초보자를-위한-vector-embeddings-가이드-timescalecom)
- [벡터 DB의 개념잡기 & LLM의 정의](#벡터-db의-개념잡기--llm의-정의)
- [머신러닝 논문정리 잘된 채널_임커밋](https://www.youtube.com/@%EC%9E%84%EC%BB%A4%EB%B0%8B)

<hr />

- [huggingface Tutorial](https://huggingface.co/docs/hub/repositories-getting-started)
- [https://huggingface.co/](https://huggingface.co/)
  - [huggingface.co 모델 다운 받는 방법](#huggingfaceco-모델-다운-받는-방법) 

<hr />

- [Ollama쓸만한거 요즘 잘 쓰는중241217](#ollama-쓸만한거)

<hr />

- C++로 구현한 머신 러닝
  - [(241217)드디어 올라옴 이걸 러스트 코드로 만들면 대박이요 ㅋㅋGN⁺: C++와 CUDA를 사용하여 처음부터 LLM 추론 엔진 만들기](#241217드디어-올라옴-이걸-러스트-코드로-만들면-대박이요-ㅋㅋgn-c와-cuda를-사용하여-처음부터-llm-추론-엔진-만들기)

<hr />

- [요즘 핫한 기술 1bits집중하자. 엔비디아 float종류 알아보기](#1bit에-집중하자-nvidia도-이제-끝이네)

<hr>

# 최신뉴스
- [(241215)LLM -> LCM에서 최적화하려는 움직임(하루가 다르게 변한다.AI Trend ㅋ)LCM: The Ultimate Evolution of AI? Large Concept Models | Discover AI](https://youtu.be/y1MG0BCf3UU?si=A70mXdj-uCFEixqE)

- [(241119)Google DeepMind has a new way to look inside an AI’s “mind”구글 딥마인드, AI ‘마음’ 들여다볼 혁신적 돌파구 마련](https://www.technologyreview.kr/%EA%B5%AC%EA%B8%80-%EB%94%A5%EB%A7%88%EC%9D%B8%EB%93%9C-ai-%EB%A7%88%EC%9D%8C-%EB%93%A4%EC%97%AC%EB%8B%A4%EB%B3%BC-%ED%98%81%EC%8B%A0%EC%A0%81-%EB%8F%8C%ED%8C%8C%EA%B5%AC-%EB%A7%88%EB%A0%A8/)

- [(241001)Should you use Rust in LLM based tools for performance?](https://bosun.ai/posts/rust-for-genai-performance/)

- [(24.7.29.) NVIDIA GPU 주도권 무너진다…? | 세계 최강 반도체 설계의 전설 짐 켈러… HBM 없는 AI 칩 출시 (Tenstorrent Wormhole) | 안될공학 - IT 테크 신기술](https://youtu.be/6NuK0y27DLY?si=NGXuB6io80X8tNhQ)

- [(240308)메타는 준비되어 있다 | LLM의 한계, JEPA | Endplan : AI 인사이트](https://youtu.be/ZxDTa_IuDuM?si=rYyaW1DBkBwBFXW3)

- [(240301)GN⁺: 1비트 LLM 시대: 비용 효율적인 컴퓨팅을 위한 삼진 파라미터 (arxiv.org)](https://news.hada.io/topic?id=13573)
- [(240131)생성형 AI 시대, 꽃피우는 ‘벡터 DB’](http://www.itdaily.kr/news/articleView.html?idxno=220008)

<hr />

- [Computer Science 관점에서 머신 러닝 이해하기](https://youtu.be/fTMMsreAqX0?si=RxgDmK-vF8GNTkUh)


<hr>

# huggingface.co 모델 다운 받는 방법[|🔝|](#link)

- https://huggingface.co/TheBloke/LLaMA-13b-GGUF

```
pip3 install huggingface-hub

huggingface-cli download TheBloke/LLaMA-13b-GGUF llama-13b.Q4_K_M.gguf --local-dir . --local-dir-use-symlinks False
```


# (241217)드디어 올라옴 이걸 러스트 코드로 만들면 대박이요 ㅋㅋ**[GN⁺: C++와 CUDA를 사용하여 처음부터 LLM 추론 엔진 만들기](<https://news.hada.io/topic?id=18295&utm_source=discord&utm_medium=bot&utm_campaign=1480>)**[|🔝|](#link)
- https://andrewkchan.dev/posts/yalm.html
  - https://github.com/andrewkchan/yalm
    - 여기서 fork해서 만듬 https://github.com/zeux/calm
  - C++와 CUDA를 사용하여 라이브러리 없이 LLM 추론 엔진을 구축하는 방법  
  - 이를 통해 LLM 추론의 전체 스택을 이해하고, 다양한 최적화가 추론 속도에 미치는 영향을 실감할 수 있음  
  - 목표 : 단일 CPU + GPU 서버에서 **단일 배치로 빠르게 추론**할 수 있도록 모델을 구현하고 **llama.cpp**보다 빠른 토큰 처...

# Run LLaMA inference on CPU, with Rust 🦀🚀🦙[|🔝|](#link)

- https://github.com/rustformers/llama-rs
  - 이게 맞는주소? https://github.com/rustformers/llm

- Inference Llama 2 in one file of pure Rust 🦀 
  - https://github.com/gaxler/llama2.rs

# Artificial_Intelligence(NLP, Natural Language Processing models and pipelines.)[|🔝|](#link)

- burn-candle[![crates.io](https://img.shields.io/crates/v/burn-candle.svg)](https://crates.io/crates/burn-candle)![Crates.io](https://img.shields.io/crates/l/burn-candle)![druidDownloads](https://img.shields.io/crates/d/burn-candle.svg)<a href="[https://github.com/guillaume-be/rust-bert](https://github.com/tracel-ai/burn/tree/main/crates/burn-candle)">
  
  - Burn is a new comprehensive dynamic Deep Learning Framework built using Rust with extreme flexibility, compute efficiency and portability as its primary goals.
    - burn https://github.com/tracel-ai/burn<img alt="githubicon" width="20px" src="https://user-images.githubusercontent.com/67513038/218287708-001511d7-1cce-42d3-92d2-4a61193b38f0.png" /></a>![star](https://img.shields.io/github/stars/tracel-ai/burn.svg)
  - https://crates.io/crates/burn-candle
  - https://github.com/tracel-ai/burn/tree/main/crates/burn-candle
  - burn https://github.com/tracel-ai/burn

- rust-bert[![crates.io](https://img.shields.io/crates/v/rust-bert.svg)](https://crates.io/crates/rust-bert)![Crates.io](https://img.shields.io/crates/l/rust-bert)![druidDownloads](https://img.shields.io/crates/d/rust-bert.svg)<a href="https://github.com/guillaume-be/rust-bert"><img alt="githubicon" width="20px" src="https://user-images.githubusercontent.com/67513038/218287708-001511d7-1cce-42d3-92d2-4a61193b38f0.png" /></a>
  ![star](https://img.shields.io/github/stars/guillaume-be/rust-bert.svg)

  - Rust native ready-to-use NLP pipelines and transformer-based models (BERT, DistilBERT, GPT2,...)

  - https://github.com/guillaume-be/rust-bert

  - https://crates.io/crates/rust-bert

# Rust MachineLearning<img alt="rustmascot" width="26px" src="https://user-images.githubusercontent.com/67513038/213403213-1b1b3efc-ce53-4825-9dfc-e9bf2956a7f4.svg" /></a>[|🔝|](#link)

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

# dfdx: shape checked deep learning in rust[|🔝|](#link)

- dfdx[![crates.io](https://img.shields.io/crates/v/dfdx.svg)](https://crates.io/crates/dfdx)![Crates.io](https://img.shields.io/crates/l/dfdx)![druidDownloads](https://img.shields.io/crates/d/dfdx.svg)<a href="https://github.com/coreylowman/dfdx"><img alt="githubicon" width="20px" src="https://user-images.githubusercontent.com/67513038/218287708-001511d7-1cce-42d3-92d2-4a61193b38f0.png" /></a>![star](https://img.shields.io/github/stars/coreylowman/dfdx.svg)

- https://github.com/coreylowman/dfdx

  - dfdx v0.11.0: ergonomic GPU accelerated deep learning ENTIRELY in rust!

    - https://coreylowman.github.io/2023/03/15/release-0.11.0.html

# Minimalist ML framework for Rust[|🔝|](#link)

https://github.com/huggingface/candle

<br>

<hr>

# ollama 쓸만한거[|🔝|](#link)

```bash
# llama3.3(가정용 컴퓨터로 405B모델을 경험 가능 지금은 아주 느리다. 241212
# New state of the art 70B model. Llama 3.3 70B offers similar performance compared to Llama 3.1 405B model.
ollama run llama3.3

# ollam 프로세서 잘 실행 되는지 확인
# pgrep ollama
6327
6521

# ollam "/bye" 로 종료 시키고 서비스 종료 시키기 
$ systemctl stop ollama.service

# 4.7GB
ollama run llama3.1

# 26GB
ollama run mixtral:8x7b

# 39GB
ollama run llama3.1:70b

# 79GB
ollama run mixtral:8x22b
```

# (C++코드로 머신러닝 잘 설명됨.)Snake learns with NEUROEVOLUTION (implementing NEAT from scratch in C++) |Tech With Nikola
- https://youtu.be/lAjcH-hCusg?si=eeEWJpy3SacoQYAb
  - 역시 핵심은 Sigmoid 함수와 Bias를 활용하는것!


<hr>

# 1bit에 집중하자 NVIDIA도 이제 끝이네[|🔝|](#link)
- Will NVIDIA Survive The Era of 1-Bit LLMs? | Finxter
  - https://youtu.be/HGbTAV8RoZQ?si=X6qZbabhOkAOj8Xr 
- Matrix Multiplication is AI - What 1.58b LLMs Mean for NVIDIA | Finxter
  - https://youtu.be/isOcqRuJkAo?si=zlzqt5gaTdc7y3LX 

||Blackwell|Hopper|
|-|-|-|
|Supported Tensor Core precisions|FP64, TF32, BF16, FP16, FP8, INT8, FP6, FP4|FP64, TF32, BF16, FP16, FP8, INT8|
|Supported CUDA* Core precisions|FP64, FP32, FP16, BF16|FP64, FP32, FP16, BF16, INT8|

- Nvidia Blackwell Deep Dive (GB 200 NVL72) $125B Revenue Projection
  - https://youtu.be/gxRGPTv82AY?si=AplrKMKzFXO5qOOw

<hr>

# NVIDIA칩 자세히 알아보기(240617)[|🔝|](#link)
- https://youtu.be/0v-W7TM6NCE?si=eVgokdiNLwQA0Tob

# NVIDIA는 16-bit Float(FP16/BF16) 부동 소수점에 최적화 되어있어서[|🔝|](#link)

- 완전히 다른 방식으로 접근하고 있다.
  - Develop optimized kernels for 1-bit operations
  - Use FPGAs or ASICs for 1-bit operations
# BitNet b1.58(This Work). vs 16-bit Float(FP16/BF16)[|🔝|](#link)
- 9min 46s 참고

# Why BitNet b1.58?[|🔝|](#link)
- Each cell only three values: 
  - { -1, 0 ,1 }
  - How many bits are needed to differentiate three equally likely states? 
## $$Log_2(3) = 1.58$$ ##

- https://youtu.be/HGbTAV8RoZQ?si=0Bu_ovLzuTI9SRWR

# (24년 04월경쯤)GN⁺: 1비트 LLM 시대: 비용 효율적인 컴퓨팅을 위한 삼진 파라미터 (arxiv.org)[|🔝|](#link)
- https://news.hada.io/topic?id=13573

<hr>

# 벡터 DB의 개념잡기 & LLM의 정의[|🔝|](#link)
- 출처 : http://www.itdaily.kr/news/articleView.html?idxno=220008
- LLM은
  - 딥러닝의 한 종류로 벡터 데이터로 이루어진 언어모델이라는 것이다. 실제 벡터 데이터로 이뤄진 LLM에는 이미 데이터들을 저장하는 벡터 DB가 내장돼 있다.
- 벡터 DB는
  - 유사한 벡터값끼리 군집을 형성한 딥러닝 모델을 학습시키기 위해선 벡터 데이터에 특화된 데이터 저장소가 필요하다. 이러한 요구에 따라 정형화된 데이터가 아닌 비정형 데이터를 빠르게 벡터화(임베딩, Embedding)해 저장하고 읽을 수 있는 벡터 DB가 등장했다.

- 벡터DB의장점 4가지
  - △벡터 데이터 처리
  - △벡터 검색 및 유사성 분석
  - △대용량 벡터 데이터 처리
  - △벡터 데이터 갱신 등에 특화된 장점을 갖고 있다.

<hr />

- 1. △벡터 데이터 처리
  - 벡터 데이터를 처리하는 데 특화돼 있다. 생성형 AI에서는 주로 벡터 데이터가 이용된다. LLM 내 토큰이 벡터화돼 내장된 벡터 DB에 저장돼 있기 때문이다. 생성형 AI에서는 주로 이미지, 음성, 텍스트 등 비정형 데이터가 벡터 형태로 변환돼 처리되는데, 벡터 DB는 벡터 데이터를 효과적으로 저장하고 관리할 수 있다.

- 2. △벡터 검색 및 유사성 분석
  - 벡터 검색 및 유사성 분석에 특화된 기능을 제공한다는 점이다. 이에 대해 EDB 측 관계자는 “생성형 AI에서는 벡터 데이터 간의 유사성을 분석하거나 벡터 데이터 검색이 필요한 경우가 많다. 벡터 DB는 이러한 요구사항에 특화된 기능을 제공해 벡터 데이터 간 유사성을 효과적으로 계산하고 검색할 수 있다. 따라서 생성형 AI에서는 벡터 DB를 통해 빠르고 정확한 벡터 검색 및 유사성 분석이 가능하다”고 설명했다.

- 3. △대용량 벡터 데이터 처리
  - 세 번째로는 대용량 벡터 데이터 처리가 가능하다는 점이다. 생성형 AI 모델은 대용량의 벡터 데이터를 다룬다. 벡터 DB는 대용량의 벡터 데이터를 효과적으로 저장하고 처리할 수 있는 기능을 제공하기 때문에 생성형 AI 모델의 대규모 데이터에 대응할 수 있다. 널리 쓰이는 관계형 데이터베이스(RDB)는 벡터 데이터의 길이나 형태의 다양성, 쿼리 및 인덱싱, 데이터 모델 무결성 제약 등 때문에 벡터 데이터를 저장하기에 적합하지 않다.

- 4. △벡터 데이터 갱신
  - 벡터 DB는 벡터 데이터의 갱신 및 쿼리에 대한 기능을 제공한다. 벡터 DB는 일반적으로 단독으로 쓰이지 않는다. 대부분의 경우 LLM을 벡터 DB가 연결된 랭체인(LangChain)이라는 언어 데이터를 수집하고 저장하는 플랫폼을 연결해 이용하는 구조다. 데이터 소스. 단어 임베딩, 벡터 DB 등을 LLM과 연결하는 매개라고 볼 수 있다. 개별로 구축된 벡터 DB에 최신의 데이터를 임베딩해 저장하면 LLM 재학습 하지 않고도 최신 데이터를 LLM에 적용할 수 있게 된다.
    - “생성형 AI의 치명적인 문제로 꼽히는 환각 현상은 상당 부분 데이터 최신화 문제 때문에 발생한다. LLM은 특정 시점까지 학습된 데이터로 구축된다. 때문에 학습 데이터를 꾸준히 최신화해야 한다. 그러나 학습 데이터를 최신화 하기 위해서는 인프라 비용, GPU 비용, 인력 투입 등 많은 비용과 시간이 필요하다. 이런 문제의 상당부분을 벡터 DB로 해결할 수 있다”

- 벡터DB의 차이점
  - 벡터 DB는 사실 타 DBMS와 큰 차이가 없다. 다만 다루는 데이터의 성격과 처리 방법이 다르다. 벡터 DB는 주로 실수(Real Number) 형태의 데이터가 포함된다. 또 실수 형태의 데이터를 기반으로 유사도가 높은 결과를 추출하기 위해 다양한 방법을 제공할 수 있다”면서 “벡터화된 데이터 간의 유사도를 측정하는 데에는 주로 ‘코사인 유사도(Cosine Similarity)’와 ‘유크리드 거리(Euclidean Distance)’ 등의 측정 방법이 활용된다. 코사인 유사도는 두 벡터 사잇각을 통해 벡터 데이터가 얼마나 유사도가 있는지 측정하는 방법이며, 유크리드 거리는 평면에서의 두 벡터값 사이의 직선거리를 측정해 값 사이의 유사도를 파악하는 방법이다”라고 설명했다.
  - “벡터 DB는 의미 기반의 쿼리를 가능하게 한다. 기존 DBMS는 각 스키마의 관계를 통해 데이터를 효율적으로 추출하는 데 중점을 두지만, 벡터 DB는 수치화된 벡터의 의미를 효과적으로 추출하는 데 중점을 둔다. 수치화된 데이터 저장과 유사도 측정을 위한 다양한 알고리즘을 제공하는 DB가 바로 벡터 DB다”



<hr />


# 초보자를 위한 Vector Embeddings 가이드 (timescale.com)[|🔝|](#link)
- https://news.hada.io/topic?id=15094&utm_source=weekly&utm_medium=email&utm_campaign=202423
  - 26P by xguru 24.05.31.
- 벡터 임베딩의 종류
  - 단어 임베딩: NLP에서 단어를 표현하며, 단어 간의 의미적 관계를 캡처함. 언어 번역, 단어 유사성, 감정 분석 등에 사용됨.
  - 문장 임베딩: 문장의 의미와 문맥을 캡처하며, 정보 검색, 텍스트 분류, 감정 분석 등에 사용됨.
  - 문서 임베딩: 보고서나 기사 같은 문서의 내용을 캡처하며, 추천 시스템, 정보 검색, 문서 유사성 및 분류 등에 사용됨.
  - 그래프 임베딩: 그래프의 노드와 엣지를 벡터 공간에 표현하며, 노드 분류, 커뮤니티 인식, 링크 예측 등에 사용됨.
  - 이미지 임베딩: 이미지의 다양한 측면을 표현하며, 콘텐츠 기반 추천 시스템, 이미지 및 객체 인식, 이미지 검색 시스템 등에 사용됨.
  - 제품 임베딩: 디지털 제품이나 물리적 제품을 표현하며, 제품 추천 및 분류 시스템, 제품 검색 등에 사용됨.
  - 오디오 임베딩: 오디오 신호의 리듬, 톤, 피치 등을 표현하며, 감정 감지, 음성 인식, 음악 추천 등에 사용됨.

- 신경망이 임베딩을 생성하는 방법
  - 표현 학습: 신경망이 고차원 데이터를 저차원 공간으로 매핑하여 중요한 특성을 보존함.
  - 훈련 과정: 신경망이 데이터를 의미 있는 임베딩으로 변환하도록 학습함. 이는 뉴런의 가중치와 바이어스를 조정하는 과정에서 이루어짐.
  - 예시: 영화 리뷰의 긍정/부정 분류를 위한 신경망에서 단어 임베딩이 학습됨. "good"과 "excellent" 같은 단어는 유사한 임베딩을 가지게 됨.

- 벡터 임베딩의 작동 원리
  - 벡터 공간: 객체나 특징을 다차원 벡터 공간의 점으로 표현하며, 유사한 항목은 가까이 위치함.
  - 거리 측정: 유클리드 거리, 코사인 유사도 등을 사용하여 벡터 간의 관계를 정량화함.
  - 예시: "cat"과 "dog"의 벡터는 "cat"과 "car"의 벡터보다 더 가까이 위치함.

- 벡터 임베딩을 활용한 개발
  - 챗봇: 사용자 쿼리에 더 잘 응답하고, 문맥적으로 관련된 응답을 생성하며, 일관된 대화를 유지함.
  - 시맨틱 검색 엔진: 키워드 매칭 대신 의미적 유사성에 기반한 검색 결과를 제공함.
  - 텍스트 분류 시스템: 문서를 구문과 단어에 따라 분류함.
  - 추천 시스템: 키워드와 설명의 유사성에 따라 콘텐츠를 추천함.

- 데이터에 대한 벡터 임베딩 생성 방법
  - 데이터 수집: 텍스트, 오디오, 이미지, 시계열 데이터 등 다양한 데이터를 수집함.
  - 데이터 전처리: 토큰화, 노이즈 제거, 이미지 크기 조정, 정규화 등 데이터를 분석에 적합하게 처리함.
  - 데이터 분할: 텍스트를 문장이나 단어로, 이미지를 세그먼트로, 시계열 데이터를 간격으로 나눔.
  - 벡터화: 각 데이터 조각을 벡터로 변환함. 텍스트 데이터는 OpenAI의 텍스트 임베딩 모델, 이미지 데이터는 CNN 모델, 오디오 데이터는 스펙트로그램 등을 사용함.
- 벡터 임베딩 저장 방법
  - 벡터 데이터베이스: 벡터 데이터를 효율적으로 저장하고 검색할 수 있는 데이터베이스 사용.
  - PostgreSQL: 벡터 데이터를 다른 관계형 데이터와 함께 저장할 수 있음. pgvector 확장을 사용하여 벡터를 저장하고 쿼리할 수 있음.

- 그 외에 좋은글
  - [임베딩(Embeddings)은 무엇이고 왜 중요한가](https://news.hada.io/topic?id=11593)
  - [머신러닝 분야의 임베딩(Embedding)에 대한 상세한 가이드](https://news.hada.io/topic?id=9316)
  - [AI에 호기심 있는 앱 개발자를 위한 좋은 시작점, Embeddings](https://news.hada.io/topic?id=14380)

<hr>

<hr>

# 역시 갓 c언어 
- **[llm.c, 이제 멀티GPU 트레이닝을 지원하며 PyTorch보다 ~7% 빠름](<https://news.hada.io/topic?id=14658&utm_source=discord&utm_medium=bot&utm_campaign=1480>)**
- Andrej Karpathy가 순수 C/CUDA로 만든 간단한 LLM 훈련 코드  
- 이제 멀티 GPU 트레이닝을 bfloat16으로 Flash Attention과 함께 수행   
- ~3000 라인의 C/CUDA 코드로 구현되었으며, 전반적으로 PyTorch보다 7% 정도까지 빠름   
- 지금까지 작업한 내용들   
  -  혼합 정밀도 훈련(bfloat16)  
  - 정규화된...

# 파이토치 bye bye 👋  존나게 구린 파이토치 ㅋㅋㅋ 그동안 참고 쓰느라 힘들었다 ㅋㅋ 더럽고 치사해서 더 공부해서 러스트로 만들어 보자 ㅋㅋ
- https://news.hada.io/topic?id=14228

# 바로 해봐야지
- https://github.com/karpathy/llm.c/discussions/344

<hr>

# MachineLearning_Tutorial
- Introduction to Deep Learning
  - https://github.com/sjchoi86/intro-dl

- Lecture notes on Bayesian deep learning
  - https://github.com/sjchoi86/bayes-nn

- 파이토치 구리지만 아직 기득권이니 공부하자( PyTorch for Deep Learning & Machine Learning – Full Course | freeCodeCamp.org
  -  https://youtu.be/V_xro1bcAuA?si=ZVKXLB8Q6kwugdm2


<hr>

# LLM -> LMM으로 패러다임 전환 중~~
- 231012_LLM은 옛말...이미지까지 학습한 'LMM' 뜬다
  - https://www.aitimes.com/news/articleView.html?idxno=154291
  - 대형언어모델(LLM)'에 이어 앞으로는 '대형멀티모달모델(LMM)
  - LLM(Large Language Models)
  - Multimodality and Large Multimodal Models (LMMs)
    - https://huyenchip.com/2023/10/10/multimodal.html
- 231014_모든 DB는 머지않아 벡터 데이터베이스가 될 것이다 (nextword.substack.com)
  - https://news.hada.io/topic?id=11263&utm_source=discord&utm_medium=bot&utm_campaign=1480
  - https://nextword.substack.com/p/vector-database-is-not-a-separate
- 231014_Llama 2 Everywhere (L2E) - 스탠드얼론, 바이너리 포터블, 부팅 가능한 Llama 2 (github.com/trholding)
  - https://news.hada.io/topic?id=11285&utm_source=discord&utm_medium=bot&utm_campaign=1480
  - https://github.com/trholding/llama2.c


<br>

# Jupyter 노트북 러스트로 빠르게 돌리기

- https://racum.blog/articles/rust-jupyter/

- First, you need to download and build the kernel itself via cargo:

```bash
$ cargo install --locked evcxr_jupyter
```

- Then, use its binary to automatically install it inside Jupyter:

```bash
$ evcxr_jupyter --install
```

<hr>

# Rust+WASM으로 이기종 Edge에서 빠르고 포터블한 Llama2 추론 실행하기 (secondstate.io)
- https://news.hada.io/topic?id=11847&utm_source=discord&utm_medium=bot&utm_campaign=1480
  - https://www.secondstate.io/articles/fast-llm-inference/
  - https://github.com/second-state/WasmEdge-WASINN-examples/tree/master/wasmedge-ggml-llama-interactive
  - LLM모델 골라 쓰기 https://huggingface.co/TheBloke

<hr>

# m1 macOS pytorch install

https://pytorch.org/get-started/locally/

<hr>

# h2oGPT - 완전한 오픈소스 GPT (github.com/h2oai)

- https://github.com/h2oai/h2ogpt
  - 한글로 된 뉴스 기사‘노트북으로도 뚝딱’··· 로컬 시스템용 LLM 도구 5종 따라잡기 https://www.ciokorea.com/news/305929?page=0,0
- https://news.hada.io/topic?id=9105

- 노트북으로도 뚝딱’··· 로컬 시스템용 LLM 도구 5종 따라잡기 (원문보기:https://www.ciokorea.com/news/305929?page=0,0#csidx306f4ef0cca5c53bd4ca4c68ff36480 )

# llama2를 파인 튜닝 하고 있는 사람들

- https://news.hada.io/topic?id=10898&utm_source=discord&utm_medium=bot&utm_campaign=1480
  - https://economiceco.tistory.com/18790

# JS강의 No Black Box Machine Learning Course – Learn Without Libraries
https://youtu.be/vDDjtwQDw2k?si=exYH6L2aHAYEqGTJ

- Machine Learning & Neural Networks without Libraries – No Black Box Course
  - https://youtu.be/3wwiOSxDAmg?si=FndfDStC4CRGoDWM

<hr>

# AlphaGo - The Movie | Full award-winning documentary

https://youtu.be/WXuK6gekU1Y?si=D9ZPN7Lxc6icN2g9

<hr>

# Building a neural network FROM SCRATCH (no Tensorflow/Pytorch, just numpy & math)

- C언어로 Tesorflow/Pythorch 라이브러리 안 쓰고 신경망 구축하기 꼭 해보자❤

https://youtu.be/w8yWXqWQYmU

<hr>

# 신경망 수학 그림으로 다 이해하기 - 복잡한 신경망도 다 이해된다 !!! 최고Why Neural Networks can learn (almost) anything | Emergent Garden

https://youtu.be/0QczhVg5HaI


<hr>


# Dalai - Automatically install, run, and play with LLaMA on your computer

- What is Dalai?

It lets you one-click install LLaMA on your machine. No need to bother building cpp files, cloning GitHub, and downloading files and stuff. Everything is automated.
Dalai is a tool in the Large Language Model Tools category of a tech stack.
Dalai is an open source tool with GitHub stars and GitHub forks. Here’s a link to Dalai's open source repository on GitHub

https://cocktailpeanut.github.io/dalai/#/

https://stackshare.io/dalai?utm_source=weekly_digest&amp;utm_medium=email&amp;utm_campaign=03292023&amp;utm_content=new_tool

- 한국에 누군가 올린 게시판 글

https://www.ddengle.com/board_free/19129866

# The Pile is a large, diverse, open source language modelling data set

https://github.com/EleutherAI/the-pile

<hr>

# brew install libtorch(macOS)

- pytorch 실행전 이거 먼저 실행할 것 !!!

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

- https://rust-ml.github.io/book/

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

# This repository is a list of machine learning libraries written in Rust. It's a compilation of GitHub repositories, blogs, books, movies, discussions, papers, etc. 🦀

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

- 은근히 코드 안 맞는다. ㅋ

https://athemathmo.github.io/rusty-machine/doc/rusty_machine/index.html

<br>

<hr>

# How to Build a Machine Learning Model in Rust

https://www.freecodecamp.org/news/how-to-build-a-machine-learning-model-in-rust/

<br>

# Machine_Learning_Rust

# 머신러닝기초
- Machine Learning Course for Beginners | freeCodeCamp.org
  - https://youtu.be/NWONeJKn6kc?si=wmh6EmpSKH1ZIzXH

- Understanding NVIDIA GPU Hardware as a CUDA C Programmer | Episode 2: GPU Compute Architecture | 0Mean1Sigma
  - https://youtu.be/1Goq8Yc3dfo?si=KgztF66DmIwP7qa9
- Awesome Production Machine Learning
  - This repository contains a curated list of awesome open source libraries that will help you deploy, monitor, version, scale, and secure your production machine learning 🚀
  - https://github.com/EthicalML/awesome-production-machine-learning 


<hr>

- Rust code들 모음
  - 최신코드 많음 
    - https://github.com/e-tornike/best-of-ml-rust
  - 좋긴 한데 옛날꺼...
    - [This repository accompanies Practical Machine Learning with Rust by Joydeep Bhattacharjee (Apress, 2020).https://github.com/Apress/practical-machine-learning-w-rust](https://github.com/Apress/practical-machine-learning-w-rust)
    - https://github.com/vaaaaanquish/Awesome-Rust-MachineLearning
