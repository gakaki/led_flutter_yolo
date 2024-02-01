## 
python3.12 -m pip install -i https://pypi.tuna.tsinghua.edu.cn/simple  --upgrade -r requirments.txt
python3.12 -m pip  install -i https://pypi.tuna.tsinghua.edu.cn/simple --upgrade -r requirments.txt

`yolo predict model=yolov8n.pt source='https://ultralytics.com/images/bus.jpg'`
/Users/macbook/.rye/shims/python -m pip  install -i https://pypi.tuna.tsinghua.edu.cn/simple --upgrade -r requirments.txt


## paddlelabel
paddlelabel --port 8000 --lan --debug # 在8000端口上运行，将服务暴露到局域网，显示详细log
paddlelabel  # 启动paddlelabel
pdlabel # 缩写，和paddlelabel完全相同

##
pip install paddlepaddle-gpu -i https://mirror.baidu.com/pypi/simple
pip install paddlepaddle -i https://mirror.baidu.com/pypi/simple
pip install "paddleocr>=2.0.1" -i https://mirror.baidu.com/pypi/simple # 推荐使用2.0.1+版本

python3.11 -m pip install --upgrade pip

export https_proxy=http://127.0.0.1:7890;export http_proxy=http://127.0.0.1:7890;export all_proxy=socks5://127.0.0.1:7890
rye config --set-bool behavior.global-python=true
rye fetch cpython@3.11.0