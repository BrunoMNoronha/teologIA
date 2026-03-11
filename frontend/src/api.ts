import axios from 'axios';

const api = axios.create({
  baseURL: 'https://vps.dev2bless.com.br/api',
  headers: {
    'Content-Type': 'application/json'
  }
});

export default api;