<script setup lang="ts">
import { nextTick, ref } from 'vue';

import api from './api';

type MessageRole = 'user' | 'assistant';

interface Message {
  role: MessageRole;
  content: string;
}

interface ChatResponse {
  resposta: string;
}

const userInput = ref('');
const messages = ref<Message[]>([]);
const isLoading = ref(false);
const scrollRef = ref<HTMLElement | null>(null);

const scrollToBottom = async () => {
  await nextTick();

  if (scrollRef.value) {
    scrollRef.value.scrollTop = scrollRef.value.scrollHeight;
  }
};

const sendMessage = async () => {
  const userText = userInput.value.trim();

  if (!userText || isLoading.value) {
    return;
  }

  messages.value.push({ role: 'user', content: userText });
  userInput.value = '';
  isLoading.value = true;

  await scrollToBottom();

  try {
    const response = await api.post<ChatResponse>('/chat', {
      usuario_id: 'bruno_web',
      texto: userText,
    });

    messages.value.push({
      role: 'assistant',
      content: response.data.resposta,
    });
  } catch {
    messages.value.push({
      role: 'assistant',
      content:
        'Amado, houve uma interrup\u00e7\u00e3o na comunica\u00e7\u00e3o com os c\u00e9us. Tente novamente.',
    });
  } finally {
    isLoading.value = false;
    await scrollToBottom();
  }
};
</script>

<template>
  <div class="mx-auto flex h-screen max-w-5xl flex-col bg-white shadow-2xl">
    <header
      class="flex items-center justify-between border-b-4 border-indigo-900 bg-indigo-700 p-6 text-white"
    >
      <div class="flex items-center gap-4">
        <div
          class="flex h-12 w-12 items-center justify-center rounded-full bg-white text-xl font-black text-indigo-700 shadow-inner"
        >
          MT
        </div>

        <div>
          <h1 class="text-2xl font-black tracking-tight">
            TEOLOG<span class="text-indigo-300">IA</span>
          </h1>
          <p class="text-xs font-medium uppercase tracking-widest text-indigo-200">
            Mestre Te&oacute;logo Digital
          </p>
        </div>
      </div>

      <div class="text-right">
        <p class="text-[10px] font-bold uppercase text-indigo-300">Powered by</p>
        <p class="text-sm font-black italic tracking-tighter">Dev2Bless</p>
      </div>
    </header>

    <main
      ref="scrollRef"
      class="flex-1 space-y-6 overflow-y-auto bg-slate-50 p-6"
    >
      <div
        v-if="messages.length === 0"
        class="flex h-full flex-col items-center justify-center space-y-4 text-slate-400"
      >
        <div
          class="flex h-20 w-20 items-center justify-center rounded-full border-2 border-dashed border-slate-200 bg-slate-100"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-10 w-10"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="1.5"
              d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"
            />
          </svg>
        </div>

        <p class="text-center font-medium italic">
          "Paz seja convosco! Como posso auxiliar em seus estudos hoje?"
        </p>
      </div>

      <div
        v-for="(msg, index) in messages"
        :key="index"
        :class="['flex w-full', msg.role === 'user' ? 'justify-end' : 'justify-start']"
      >
        <div
          :class="[
            'max-w-[85%] rounded-2xl p-4 text-sm leading-relaxed shadow-sm',
            msg.role === 'user'
              ? 'rounded-tr-none bg-indigo-600 text-white'
              : 'rounded-tl-none border border-slate-200 bg-white text-slate-800',
          ]"
        >
          {{ msg.content }}
        </div>
      </div>

      <div v-if="isLoading" class="flex justify-start">
        <div
          class="flex gap-1 rounded-2xl border border-slate-200 bg-white p-4 shadow-sm"
        >
          <div class="h-1.5 w-1.5 animate-bounce rounded-full bg-indigo-400"></div>
          <div
            class="h-1.5 w-1.5 animate-bounce rounded-full bg-indigo-400 [animation-delay:-0.15s]"
          ></div>
          <div
            class="h-1.5 w-1.5 animate-bounce rounded-full bg-indigo-400 [animation-delay:-0.3s]"
          ></div>
        </div>
      </div>
    </main>

    <footer class="border-t border-slate-100 bg-white p-6">
      <form class="flex items-center gap-4" @submit.prevent="sendMessage">
        <input
          v-model="userInput"
          type="text"
          placeholder="Escreva sua pergunta aqui..."
          class="flex-1 rounded-xl border-none bg-slate-100 p-4 transition-all placeholder:text-slate-400 focus:ring-2 focus:ring-indigo-500"
          :disabled="isLoading"
        />

        <button
          type="submit"
          class="flex items-center gap-2 rounded-xl bg-indigo-700 px-6 py-4 font-bold text-white transition-all hover:bg-indigo-800 disabled:bg-slate-200 disabled:text-slate-400"
          :disabled="isLoading || !userInput.trim()"
        >
          <span>ENVIAR</span>
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-5 w-5"
            viewBox="0 0 20 20"
            fill="currentColor"
          >
            <path
              d="M10.894 2.553a1 1 0 00-1.788 0l-7 14a1 1 0 001.169 1.409l5-1.429A1 1 0 009 15.571V11a1 1 0 112 0v4.571a1 1 0 00.725.962l5 1.428a1 1 0 001.17-1.408l-7-14z"
            />
          </svg>
        </button>
      </form>
    </footer>
  </div>
</template>
