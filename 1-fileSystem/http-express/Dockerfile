FROM node:alpine

WORKDIR /usr/src/

COPY package*.json .

RUN npm install 
RUN npm install typescript 

COPY . . 

EXPOSE 3000

CMD [ "npm","run","dev" ]


