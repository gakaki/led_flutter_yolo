import 'dart:convert';

import 'package:dio/dio.dart';
import 'package:flutter/material.dart';
import 'package:led_text/page/login_page.dart';
import 'package:led_text/page/theme/app_colors.dart';
import 'package:led_text/widgets/welcome_widget.dart';

// 实验记录列表
class ExperimentListPage extends StatefulWidget {
  ExperimentListPage({Key? key}) : super(key: key);

  final List<String> items = new List<String>.generate(10000, (i) => "Item $i");

  @override
  _ExperimentListPageState createState() => _ExperimentListPageState();
}

class _ExperimentListPageState extends State<ExperimentListPage> {  // 用于存储 HTTP 请求的数据
  List<Item> _items = [];

  get jsonDecode => null;

  // 调用 HTTP Service 获取数据
  Future<void> _fetchData() async {
    try {
      // 替换为你实际的 HTTP Service 地址
      final response = await Dio().get('https://jsonplaceholder.typicode.com/todos/1');

      if (response.statusCode == 200) {
        final data = response.data;
        var model = Item.fromJson(data);
        print(model.userId);

        // for (var item in json) {
        //   var model = Item.fromJson(item);
        //   print(model.userId);
        // }

        final items = data.map<Item>((json) => Item.fromJson(json)).toList();

        // 更新数据
        setState(() {
          _items = items;
        });
      } else {
        // 处理错误情况
        throw Exception('HTTP 请求失败');
      }

    } catch (e) {
      // 处理异常情况
      print(e);
    }
  }


  // void _fetchData() async {
  //   try {
  //     Response response = await Dio().get("https://jsonplaceholder.typicode.com/todos/1");
  //     print(response);
  //   } catch (e) {
  //     print(e);
  //   }
  // }

  @override
  void initState() {
    super.initState();

    _fetchData();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('实验室列表'),
      ),
      body: ListView.builder(
        itemCount: _items.length,
        itemBuilder: (context, index) {
          return ListTile(
            title: Row(
              children: [
                Text(_items[index].userId.toString()),
                Text(_items[index].id.toString()),
              ],
            ),
            subtitle: Text(_items[index].title.toString()),
            trailing: Text(_items[index].completed.toString()),
            isThreeLine: true,
            leading: Icon(Icons.person),
            onTap: () {
              // 跳转到另一个页面，并传递姓名参数
              Navigator.pushNamed(context, '/second_page', arguments: {'name': _items[index].title});
            },
          );
        },
      ),
    );
  }
}

class Item {
  String? userId;
  String? id;
  String? title;
  String? completed;

  Item({this.userId, this.id, this.title, this.completed});

  Item.fromJson(Map<String, dynamic> json) {
    userId = json['userId'].toString();
    id = json['id'].toString();
    title = json['title'].toString();
    completed = json['completed'].toString();
  }

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = new Map<String, dynamic>();
    data['userId'] = this.userId;
    data['id'] = this.id;
    data['title'] = this.title;
    data['completed'] = this.completed;
    return data;
  }
}