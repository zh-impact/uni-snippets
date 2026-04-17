from openai import OpenAI

RESUME_PARSE_PROMPT = """You are a career coach specializing in resume analysis and parsing.

Analyze the following resume text and extract structured information into four categories.

Output format: JSON with exactly these fields:
{
  "positions": [
    {
      "company": "Company name",
      "title": "Job title",
      "department": "Department or team (if mentioned)",
      "startDate": "Start date (YYYY-MM or YYYY format)",
      "endDate": "End date (YYYY-MM, YYYY, or 'Present' for current)",
      "isCurrent": true/false,
      "description": "Job description and responsibilities (bullet points with newlines)"
    }
  ],
  "achievements": [
    {
      "rawInput": "Original achievement text",
      "situation": "Context or background",
      "task": "Responsibility or challenge",
      "action": "Specific actions taken",
      "result": "Quantifiable outcomes or impact",
      "summary": "One-line executive summary",
      "tags": ["tag1", "tag2"],
      "dateRange": "Date range if specified"
    }
  ],
  "skills": [
    {
      "name": "Skill name",
      "category": "Technical or Soft",
      "proficiency": 3,
      "lastUsed": "When the skill was last used (e.g., '2024', 'Current')"
    }
  ],
  "education": [
    {
      "institution": "School or university name",
      "degree": "Degree or certificate (e.g., B.S., M.S., PhD)",
      "field": "Field of study or major",
      "startDate": "Start date (YYYY or YYYY-MM)",
      "endDate": "End date (YYYY or YYYY-MM)"
    }
  ]
}

Rules:
- Extract ALL work experience entries, do not omit any
- For achievements, use the STAR method where possible; use rawInput for anything that doesn't fit STAR
- List ALL skills mentioned - both technical and soft skills
- Estimate proficiency on a 1-5 scale (1=beginner, 5=expert) based on context
- Extract ALL education entries
- Use empty arrays for categories with no data, never omit keys
- Use "Present" for end dates of current positions
- Return valid JSON only, no additional text or markdown formatting"""

RESUME_TEXT = """黄志
前端开发工程师 · 技术管理
广州
 1991-11  (+86) 134-1811-9795  work.huang@qq.com  zh-impact
专业技能
前端开发 HTML, CSS/SASS/LESS, JS/TS, React/Next.js/Redux/MobX, Jest/Testing Library/Cypress
工程化 Webpack/Vite, ESLint, CircleCI/Github Actions
综合技能 Node/Express/Koa, Python/Flask, Postgres/Redis
开发环境 macOS/Linux/bash/zsh/Git/VSCode/Docker
工作经历
Compass, Inc. 远程工作
Frontend Engineer/Engineer Manager 2021/03 - 2024/07
Compass 是美国的房地产交易平台，为房产经纪人提供全面的工具和服务支持，助力其顺利完成房屋交易。
• 参与 CoreTeam 的地产搜索面板开发工作，显著提升了平台的用户体验和搜索效率
• 加入收购的 Glide 团队，成功升级并迁移了 Glide 前端依赖，主导将原单体项目拆分为前后端分离架构，大幅提高了系统
的可维护性和性能
• 负责管理并积极参与 Plex Payments 和 Compass Service 项目的开发任务，确保项目按时交付，并优化了支付流程，减少
了处理时间达 20%
• 日常负责 CodeReview, 作为 Plex External Maintainer, 积极参与 OnCall(datadog+opsgenie) 及 Proposal Review 工作
广州品快信息科技有限公司 广州
前端开发工程师 2020/07 - 2020/08
• 参与基于企业微信的私域流量管理平台开发
• 开发数据遥测客户端 SDK
广州酷狗计算机科技有限公司 广州
Frontend Developer/Team Leader 2018/06 - 2020/03
• 负责直播网站功能页面及直播间核心功能的迭代开发，确保功能稳定性和用户体验优化
• 独立负责 APP 内嵌 H5 组件的开发与维护，提升了组件加载速度，增强了用户交互体验
• 维护并优化公司内网开发环境，包括 Verdaccio, GitLab-CI 和 Sentry 等平台
• 参与业务中台开发，如微前端组件发布系统，可视化平台开发
广东橙材电子商务有限公司 广州
前端开发工程师 2016/11 - 2018/02
公司主要业务为大宗商品期货及现货交易
• 负责优点价 SaaS 商城开发。优点价商城官网开发，利用 Express 与基于字符串的模板引擎，实现服务端渲染
• 利用 Scrapy 爬取行业资讯，并部署 ElasticSearch 进行资讯搜索
乐视致新电子科技有限公司 北京
前端开发工程师 2015/07 - 2015/12
• 负责维护和更新基于自有组件搭建的运营管理后台，该平台采用前后端分离架构开发
• 参与乐视艺术家开放平台开发。独立负责数据统计平台的开发工作
深圳兰帕德策划设计有限公司 深圳
前端开发工程师 2013/06 - 2014/10
该公司主要为腾讯互娱、财付通、京东、华为等大厂提供外包设计及前端开发
• 负责腾讯互娱等活动专题页面的开发, 利用 CSS 和 JS 制作常见动画效果, 快速交付页面
• 派驻华为, 开发华为云服务前端, 配合后端使用 Ajax 对接数据接口, 初步进行前后端分离. 实现完整应用前端逻辑

-- 1 of 3 --

项目经历
Glide 团队项目升级迁移与 Compass 平台集成
项目介绍：Glide 是一个管理房地产交易合同文档的平台，已被 Compass 收购。原架构为单体仓库，采用 Flask + React 开
发，前后端深度耦合，后端控制流程，前端根据后端返回参数渲染表单。
工作内容：
• 成功升级前端依赖组件，特别是 AntD 跨版本升级，解决了表单和基础组件样式兼容性问题。通过 Less 变量编译，实现
了符合 Compass 设计语言的样式版本，在 Glide 独立网站与 Compass 内嵌页面中展示不同主题风格
• 将路由从 Router5 迁移到 React-Router，并升级 MobX，确保迁移过程中的稳定性，修复所有相关报错
• 参与 Flask 后端开发，编写格式化处理函数，支持 Google OCR 处理 PDF 中的不同表单元素。记录并整理处理流程文档，
便于后续开发人员快速上手
• 完善项目基础设施，将前端构建流程从 Drone 拆分至 CircleCI，集成 ESLint 和单元测试覆盖率检查，提升代码质量与可
靠性
• 初期以 iFrame 形式嵌入 Compass，后期重构为微前端 package，直接在 Compass 中加载执行，显著提升首次加载效率
PLEX Payments 功能迭代
项目介绍：该项目旨在通过功能升级改造，优化 Compass 对滞留应收款项的回收流程，加速资金回流并降低风险。
工作内容：
• 担任 EM 全面负责中区项目管理工作，定期与美区团队进行跨区域沟通汇报，深入研读产品与技术文档，确保精准理解
项目需求与开发任务
• 对接 Stripe 与 Contentful 第三方平台 API，实现其与现有系统的无缝集成，提升整体业务协同效率
• 利用 AWS Secrets 管理应用密钥，通过 Node 层进行第三方服务的转发处理，有效保障系统安全与高可用性
• 运用 Poker Planning 进行任务拆分与点数估算，并在每个 Sprint 结束后组织回顾会议（Retro），不断优化团队协作与开发
流程
酷狗直播 App 通用内嵌页启动框架
项目介绍：为提升开发效率，本项目针对 App 内频繁嵌入的 H5 活动页面进行研发，旨在优化开发与调试流程，使活动页
面的集成与维护更加便捷高效。
工作内容：
• 与移动端开发团队密切协作，制定并调试底层 WebView 与 App 间的调用协议，有效解决兼容性问题，确保数据交互顺畅
• 设计并实现常驻启动管理器，监听 App 触发的调用事件，并通过定时器实时监控活动页面的生命周期及优先级，从而精
准控制页面的加载、展示、隐藏与卸载，全面提升用户体验与系统性能
酷狗直播微前端系统
项目介绍：针对直播间中频繁出现的浮窗活动页面，为了提升开发效率及灵活管理活动上线下线状态，专门研发了该系统，
实现了开发环境优化、组件独立以及动态配置管理。
工作内容：
• 开发环境与组件优化：原有直播间开发环境依赖 vagrant 启动，流程繁重。通过将活动组件独立抽离，并采用简化的
webpack 配置，实现高效开发。同时提供 mock 组件以便在开发过程中模拟获取直播间关键数据，极大降低了开发门槛
• 后端组件发布系统：参与基于 Node.js 的后端发布系统开发，利用 JWT 实现内网鉴权，并与 GitLab-CI 对接，自动获取项
目编译后的产出信息，推动持续集成流程
• 动态配置与活动管理：在后台管理面板配置各项活动的上线/下线状态后，系统自动编译生成静态 JSON 配置数据，并推
送至 CDN，实现活动页面状态的统一控制与灵活调度
• 测试服务器维护与监控：负责内网开发测试服务器的日常维护工作，编写 Shell 脚本定时监控磁盘、CPU 与内存状态；部
署 Sentry 错误监控平台，并集成企业微信消息提醒，确保系统稳定性与及时响应

-- 2 of 3 --

其他经历
lablabai Hackthon
项目介绍：本项目打造了一款基于 AI 的聊天式面试助手，模拟真实面试场景，由 AI 担任面试官，依据求职者提供的基本信息及上传的
简历自动开启面试流程，并在结束后输出综合评分与反馈，旨在让求职者预先联系求职过程。
开发内容：读取用户上传的 PDF 内容，提取简历内容作为 prompt 原料，通过 whisper 让用户以实时语音聊天的方式进行对话，调用
langchain 与大语言模型进行聊天补全，开启多轮对话。
完赛证书：https://lablab.ai/u/@zhi_huang918/cljpux1wu012zlb0hs344fa04
Open Video Maker
项目介绍：本项目集成自动化浏览器操作、屏幕录制和语音合成技术，实现从网页操作录制到视频成品生成的全流程自动化。通过
Puppeteer 控制浏览器操作，借助 xvfb 与 ffmpeg 进行视频录制，再利用微软语音合成服务结合 SSML 标记生成自然语音，最终将视频与
音频合成，形成具有高质量视听效果的成品视频。
项目地址：https://github.com/Open-Video-Maker
自由画板项目
项目介绍：该项目基于 react-konva 构建了一款功能全面的白板应用，旨在提供直观、灵活的绘图体验。应用支持绘制直线、轨迹、矩形、
椭圆、文字等多种图形，同时集成了擦除、撤销以及画板内容保存等功能。
项目地址：https://github.com/zh-impact/x-annotate
教育经历
衡阳师范学院
电子信息工程 本科 2009/09 - 2013/06

-- 3 of 3 --"""

def main():
    client = OpenAI(
        api_key="nTah8R4SSKN5EyuTwWBXwg30UYXjrHfH2TXhUF7pSy0_bv_EhT2zULN_AlyGIEdNHueTtZHwoEgiN38uxzayZQ",
        base_url="https://www.sophnet.com/api/open-apis",
    )

    response = client.chat.completions.create(
        model="DeepSeek-V3.2",
        messages=[
            {"role": "system", "content": RESUME_PARSE_PROMPT},
            {"role": "user", "content": RESUME_TEXT},
        ],
        response_format={"type": "json_object"},
    )

    if response.choices[0].message.content:
        with open("output.json", "w") as f:
            f.write(response.choices[0].message.content)
    else:
        print("No content in response")


if __name__ == "__main__":
    main()
