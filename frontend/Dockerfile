# ベースイメージ
FROM node:20-alpine AS base
WORKDIR /app

# 依存関係のインストール
FROM base AS deps
COPY package.json package-lock.json ./
RUN npm ci

# ビルダーステージ
FROM base AS builder
COPY --from=deps /app/node_modules ./node_modules
COPY . .
RUN npm run build

# ランナーステージ
FROM base AS runner
ENV NODE_ENV=production

# 非rootユーザーを作成
RUN addgroup --system --gid 1001 nodejs && \
    adduser --system --uid 1001 nextjs

# 必要なファイルのみをコピー
COPY --from=builder /app/public ./public
COPY --from=builder /app/.next/standalone ./
COPY --from=builder /app/.next/static ./.next/static

# 適切な権限を設定
RUN chown -R nextjs:nodejs /app
USER nextjs

# ポートを公開
EXPOSE 3000

# 環境変数を設定
ENV PORT=3000
ENV HOSTNAME=0.0.0.0

# アプリケーションを実行
CMD ["node", "server.js"]
