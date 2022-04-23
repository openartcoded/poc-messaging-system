FROM node:16.10
WORKDIR /app
COPY package.json package-lock.json ./
RUN npm i
COPY publish.js .
CMD [ "node", "publish.js" ]