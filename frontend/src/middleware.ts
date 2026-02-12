import { NextRequest, NextResponse } from 'next/server';

// 認証が必要なパス
const protectedPaths = ['/'];

// 認証済みユーザーがアクセスすべきでないパス
const authPaths = ['/login', '/register'];

export function middleware(request: NextRequest) {
  const { pathname } = request.nextUrl;
  const refreshToken = request.cookies.get('refresh_token');

  // 保護パスに未認証でアクセスした場合はログインページへリダイレクト
  const isProtectedPath = protectedPaths.some(
    (path) => pathname === path || pathname.startsWith(path + '/')
  );
  // ただし、ログインページとユーザー登録ページは除外
  const isAuthPath = authPaths.some((path) => pathname === path);

  if (isProtectedPath && !isAuthPath && !refreshToken) {
    const loginUrl = new URL('/login', request.url);
    loginUrl.searchParams.set('from', pathname);
    return NextResponse.redirect(loginUrl);
  }

  // 認証済みでログインページとユーザー登録ページにアクセスした場合はTOPページへリダイレクト
  if (isAuthPath && refreshToken) {
    return NextResponse.redirect(new URL('/', request.url));
  }

  return NextResponse.next();
}

export const config = {
  matcher: [
    // Next関連やfaviconなどは除外
    '/((?!_next/static|_next/image|favicon.ico).*)',
  ],
};
