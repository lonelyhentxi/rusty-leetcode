object ExpensiveResource {
  lazy val resource: Int = init()
  def init(): Int = {
    0
  }
}

// 如果需要以延迟的方式初始化某值，并且会重复计算，则需要使用惰性赋值
/*
  通过保护式来实现惰性值。当客户代码引用了惰性值，保护式会拦截引用并检查此时是否需要初始化惰性。
  虽然只有第一次访问需要初始化，因此保护式检查只有第一次引用才是有用的
  惰性值具有额外的开销，因此当保护式带来的额外开销小于初始化的开销时，或者将某些值惰性化能简化初始化过程
  并确保执行顺序满足依赖条件时，才应该使用惰性值
 */