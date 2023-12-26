# Ollama

- https://github.com/jmorganca/ollama


# Multiline input

For multiline input, you can wrap text with """:

```
>>> """Hello,
... world!
... """
I'm a basic program that prints the famous "Hello, world!" message to the console.
```


# First Ollama

- ```curl https://ollama.ai/install.sh | sh```

```
curl https://ollama.ai/install.sh | sh
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100  7983    0  7983    0     0   9570      0 --:--:-- --:--:-- --:--:--  9560
>>> Downloading ollama...
######################################################################## 100.0%##O#-#
######################################################################## 100.0%
>>> Installing ollama to /usr/local/bin...
[sudo] password for **** : 
>>> Creating ollama user...
>>> Adding current user to ollama group...
>>> Creating ollama systemd service...
>>> Enabling and starting ollama service...
Created symlink /etc/systemd/system/default.target.wants/ollama.service → /etc/systemd/system/ollama.service.
>>> NVIDIA GPU installed.
```

# ```ollama run mistral```

```

// 이게 8x7b모델
ollama run mixtral

ollama run mistral
```

- GN⁺: Mistral AI, Llama 2 70B 모델보다 뛰어난 Mixtral 8x7B 모델 공개 (mistral.ai)
  - https://news.hada.io/topic?id=12296&utm_source=weekly&utm_medium=email&utm_campaign=202351
