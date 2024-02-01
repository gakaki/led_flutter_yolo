## troubleshooting

flutter Another exception was thrown: A RenderFlex overflowed by xx pixels on the bottom
A RenderFlex overflowed by xxx pixels on the bottom
示例图片： 底部键盘弹出时。高度异常显示

以上异常 ，一般为高度不够，直接用了Column 这种不会上下滚动的组件，外边包一层SingleChildScrollView 让页面滚动即可

