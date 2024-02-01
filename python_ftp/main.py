import logging
import os
from pyftpdlib.authorizers import DummyAuthorizer
from pyftpdlib.handlers import FTPHandler
from pyftpdlib.servers import FTPServer
# https://github.com/PaddlePaddle/PaddleOCR/blob/release/2.7/StyleText/README.md#Quick_Start
# uvicorn main:app --reload
# pip install -i https://pypi.tuna.tsinghua.edu.cn/simple "fastapi[all]" "uvicorn[standard]"
# https://github.com/cgcel/PaddleOCRFastAPI/blob/master/README_CN.md
# https://cloud.tencent.com/developer/article/1758809
# https://realpython.com/fastapi-python-web-apis/
# https://juejin.cn/post/7085653337365807112

## https://www.youtube.com/watch?v=W0IwfbrdCC4&ab_channel=jomutlo
# https://crates.io/crates/hot-lib-reloader
# https://robert.kra.hn/posts/hot-reloading-rust/
def get_path(directory_name):
    current_directory = os.getcwd()
    full_path = os.path.join(current_directory, directory_name)
    return full_path

logging.basicConfig(level=logging.DEBUG)

current_ftp_cam_path = get_path("ftp_cam")
print("当前ftp cam路径",current_ftp_cam_path)

authorizer = DummyAuthorizer()

authorizer.add_user("root", "root", current_ftp_cam_path, perm="elradfmwMT") //elradfmw
# authorizer.add_anonymous("/home/nobody")

handler = FTPHandler
handler.authorizer = authorizer
handler.log_prefix = 'XXX [%(username)s]@%(remote_ip)s'

server = FTPServer(("0.0.0.0", 21), handler)
server.serve_forever()




import os
from fastapi import FastAPI, Request
from fastapi.responses import HTMLResponse
from fastapi.staticfiles import StaticFiles
from fastapi.templating import Jinja2Templates
# https://github.com/tauri-apps/tray-icon
# https://github.com/olback/tray-item-rs
# iced

app = FastAPI()

app.mount("/static", StaticFiles(directory="static"), name="static")


templates = Jinja2Templates(directory="templates")
# python3 -m pyftpdlib -w --user=username --password=password

@app.get("/static", response_class=HTMLResponse)
def list_files(request: Request):

    files = os.listdir("./static")
    files_paths = sorted([f"{request.url._url}/{f}" for f in files])
    print(files_paths)
    return templates.TemplateResponse(
        "list_files.html", {"request": request, "files": files_paths}
    )
