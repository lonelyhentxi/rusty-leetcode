﻿using System;
using System.Collections.Generic;
using System.Text;

namespace essential_cs.classes
{
    internal class Common
    {
        public static void Main()
        {
            {
                // # 类的定义和实例化
                // C# 不支持隐式确定性资源清理（在编译期确定的位置进行隐式对象析构）
                // C# 通过 using 语句支持显式确定性资源清理
                // C# 通过终结器支持隐式非确定性资源清理
            }
            {
                // # 实例字段
                {
                    // ## 实例字段的声明
                    // 在声明时可以设置字段的初始值
                }
                // ## 实例字段的访问，略
            }
            {
                // # 实例方法
                // 不需要显式的 this
                // 使用编码风格来避免歧义
                // 有时候需要使用 this 传递当前正在执行的对象的引用
            }
            {
                // # 访问修饰符
                // 五个级别的访问修饰符
                /*
                 * - public 外部
                 * - private 私有
                 */
            }
            {
                // # 属性
                // 实现封装和访问
                {
                    // ## getter，setter
                    // 见本类中访问器和修改器的实现
                }
                {
                    // ## 自动实现的属性，则不需要有 _Xxx 形式的私有属性
                    var common = new Common
                    {
                        Name = "common name",
                        Age = 10
                    };
                    Console.WriteLine(common.Name);
                    Console.WriteLine(common.Age);
                    // 自动实现的属性可以初始化赋值
                }
                // ## 属性和字段的设计编码规范，略
                {
                    // ## 提供属性验证
                    // 调用 ArgumentException 或者 ArgumentNullException 构造器时，要为 paramName 参数传递 “value”
                    // 为了适用于变化可以使用 nameof 得到名字
                }
                {
                    // ## 只读属性和只写属性
                    // 只声明 get 和 set
                    // 允许只读、只写的属性的初始化
                }
                {
                    // ## 为取值方法和赋值方法指定访问修饰符
                    // 内部的修饰符一定比外部修饰符更加严格（大于等于）
                }
                // ## 不能将属性作为 ref 和 out 参数值使用
            }
            {
                // # 构造器
                // ## 构造器的声明，略
                // ## 如果没有定义任何构造器，编译器会生成默认构造器
                {
                    // ## 对象初始化器
                    // C# 3.0 添加了对象初始化器
                    // 3.0 还同时新建了集合初始化器
                    // 终结器会在所有引用消失的时候发挥作用
                }
                // ## 构造器也可以重载
                {
                    // ## 构造器链
                    // 可以使用 this 调用构造器链
                    // C# 3.0 引入了匿名类型
                }
            }
            {
                // # 静态成员
                // 未初始化的静态成员将获得默认值
                // 数据能够和类和实例相关联
            }
            // # 静态方法，略
            {
                // # 静态构造器
                // 静态构造器用于对类进行初始化
                // 不发生显式调用，只发生在首次访问类时
                // 在静态构造器发生异常，会造成在应用程序剩余的生命周期内无法使用
                /*
                 * 静态构造器会在对类的任何成员进行首次访问之前执行，因此编译器会添加代码来检查静态成员和构造器，确保静态构造器先运行
                 * 没有静态构造器会初始化为他们的默认值，而且不会添加检查，因此每个静态变量会在他们访问之前被初始化
                 */
            }
            // # 静态属性，略
            {
                // # 静态类
                // 静态类不能包含实例变量
                // 静态类会被标记为 sealed 和 abstract 类
            }
            {

                // # 拓展方法
                /*
                 * C# 3.0 引入了拓展方法的概念，能模拟为不同类创建实例方法
                 * 只需要更改静态方法的第一参数为要拓展的类型，并添加上 this 关键字
                 * 拓展方法的修饰级必须开放于被拓展的类
                 * 通过继承来特化类型，要优于使用拓展方法
                 */
                var self = new Self();
                Console.WriteLine(self.ExtendedIsSelf());
            }
            {
                // # 封装数据
                {
                    // ## const 
                    // 类似 C++ 的 constexpr
                }
                {
                    // ## public 常量应该是恒定值
                    // 因为假如被编译到引用程序集中，则会被写为常量值
                }
                {
                    // ## readonly
                    // readonly 只能用于字段，每个实例的 readonly 字段都可以修改
                }
            }
            {
                // # 嵌套类
                // 嵌套类的访问可以从类作用域的范围
                // 嵌套类的 this 是自身，但是可以访问包容类的任何成员
                // JAVA 内部类的是同包容类实例关联的对象
            }
            {
                // # 分部类
                // C# 2.0 开始，使用 partial 声明，则可以在多个文件中定义一个类
                // 分部类并不允许对编译好的类进行拓展，只是把内容分配在多个文件中
                // C# 3.0 开始，引入了分部方法，必须定义在分部类内
                // 通过声明和实现分离实现，必须是 void，禁止 out，可以 ref
            }
        }

        private string _Name;

        public string Name
        {
            get { return _Name; }
            set { _Name = value; }
        }

        public int Age { get; set; } = 10;
    }
}