FROM node:16-alpine

WORKDIR /usr/src/app

COPY package*.json ./

RUN npm install

COPY . .

EXPOSE 3000

CMD ["sh", "-c", "if [ \"$TEST_ENV\" = 'local' ]; then npm run test-local; else npm run test-testnet; fi"]
