import 'package:flutter/material.dart';
import 'package:led_text/page/webf_page.dart';

import 'page/welcome_page.dart';
import 'page/experiment_list.dart';


void main() {
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      debugShowCheckedModeBanner: false,
      home: ExperimentListPage(),
      // home: WebFPage(),
    );
  }
}
