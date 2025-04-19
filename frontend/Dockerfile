# ベースイメージ
FROM node:20-alpine
WORKDIR /app

# 依存関係のインストール
COPY package.json package-lock.json ./
RUN npm ci

# アプリケーションのソースコードをコピー
COPY . .

# 非rootユーザーを作成
RUN addgroup --system --gid 1001 nodejs && \
    adduser --system --uid 1001 nextjs

# 適切な権限を設定
RUN chown -R nextjs:nodejs /app
USER nextjs

# ポートを公開
EXPOSE 3000

# 環境変数を設定
ENV PORT=3000
ENV HOSTNAME=0.0.0.0
ENV NODE_ENV=development

# 開発サーバーを実行
CMD ["npm", "run", "dev"]
