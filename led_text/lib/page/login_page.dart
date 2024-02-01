import 'package:flutter/material.dart';
import 'package:led_text/page/theme/app_style.dart';
import 'package:led_text/widgets/login_widget.dart';

/// 登录页面
class LoginPage extends StatefulWidget {
  LoginPage({Key? key}) : super(key: key);

  @override
  _LoginPageState createState() => _LoginPageState();
}

class _LoginPageState extends State<LoginPage> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
        backgroundColor: Colors.white,
        body: SingleChildScrollView(
          child: Stack(
            children: [
              Image.asset('assets/images/bg_login_header.png'),
              Column(
                children: [
                  SizedBox(height: 320),
                  ClipPath(
                    clipper: LoginClipper(),
                    child: LoginBodyWidget(),
                  ),
                ],
              ),
              Positioned(
                top: 64,
                left: 28,
                child: BackIcon(),
              )
            ],
          ),
        ));
  }
}

/// 登录页面内容体
class LoginBodyWidget extends StatelessWidget {
  LoginBodyWidget({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      color: Colors.white,
      width: double.maxFinite,
      padding: EdgeInsets.all(32),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          SizedBox(height: 86),
          Text(
            '登录',
            style: kTitleTextStyle,
          ),
          SizedBox(height: 20),
          Text(
            '用户名',
            style: kBodyTextStyle,
          ),
          SizedBox(height: 4),
          LoginInput(
            hintText: '用户名',
            prefixIcon: 'assets/icons/icon_email.png',
          ),
          SizedBox(height: 16),
          Text(
            '密码',
            style: kBodyTextStyle,
          ),
          SizedBox(height: 4),
          LoginInput(
            hintText: '密码',
            prefixIcon: 'assets/icons/icon_pwd.png',
            obscureText: true,
          ),
          SizedBox(height: 32),
          LoginBtnIconWidget(),
          SizedBox(height: 32),
        ],
      ),
    );
  }
}
