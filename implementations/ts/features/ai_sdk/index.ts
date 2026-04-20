import { stdin as input, stdout as output } from 'node:process'
import readline from 'node:readline/promises'
import { createOpenAICompatible } from '@ai-sdk/openai-compatible'
import { generateText, type ModelMessage } from 'ai'

const provider = createOpenAICompatible({
  name: 'sophnet',
  apiKey:
    process.env.SOPHNET_API_KEY ||
    'nTah8R4SSKN5EyuTwWBXwg30UYXjrHfH2TXhUF7pSy0_bv_EhT2zULN_AlyGIEdNHueTtZHwoEgiN38uxzayZQ',
  baseURL: 'https://www.sophnet.com/api/open-apis/v1',
  includeUsage: true,
})

const beforeUserInput = (messages: ModelMessage[]) => {}
const afterUserInput = (messages: ModelMessage[]) => {}
const beforeModelResponse = (messages: ModelMessage[]) => {}
const afterModelResponse = (messages: ModelMessage[]) => {}

const rl = readline.createInterface({ input, output })

const messages = [{ role: 'system', content: 'You are a helpful assistant.' }] as ModelMessage[]
while (true) {
  beforeUserInput(messages)
  const answer = await rl.question(messages.length > 1 ? '> ' : 'Enter your first question: ')
  afterUserInput(messages)
  messages.push({ role: 'user', content: answer })
  beforeModelResponse(messages)
  const { text } = await generateText({
    model: provider('DeepSeek-V3.2'),
    messages,
  })
  console.log(text)
  afterModelResponse(messages)
  messages.push({ role: 'assistant', content: text })
}
