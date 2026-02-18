'use client';

import { useForm } from 'react-hook-form';
import { useAuth } from '@/contexts/AuthContext';
import { useRouter, useSearchParams } from 'next/navigation';
import { Suspense, useState } from 'react';
import { LoginFormValues, loginSchema } from '@/schemas/auth';
import { zodResolver } from '@hookform/resolvers/zod';
import { AxiosError } from 'axios';
import { getMessage, Message } from '@/constants/message';
import {
  Box,
  Button,
  Card,
  Container,
  Flex,
  Heading,
  Link,
  Text,
  TextField,
} from '@radix-ui/themes';
import NextLink from 'next/link';

function LoginForm() {
  const { login } = useAuth();
  const router = useRouter();
  const searchParams = useSearchParams();
  const [serverError, setServerError] = useState('');

  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<LoginFormValues>({
    resolver: zodResolver(loginSchema),
  });

  const onSubmit = async (values: LoginFormValues) => {
    setServerError('');
    try {
      await login(values.email, values.password);
      const from = searchParams.get('from') || '/';
      router.push(from);
    } catch (error) {
      if (error instanceof AxiosError && error.response?.status === 401) {
        setServerError(
          getMessage(Message.E0004, 'メールアドレス', 'パスワード')
        );
      } else {
        setServerError(getMessage(Message.E0005, 'ログイン'));
      }
    }
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)}>
      <Flex direction="column" gap="4">
        {serverError && (
          <Text color="red" size="2">
            {serverError}
          </Text>
        )}

        <Box>
          <Text as="label" size="2" weight="medium" mb="1">
            メールアドレス
          </Text>
          <TextField.Root
            type="email"
            placeholder="email@example.com"
            {...register('email')}
          />
          {errors.email && (
            <Text color="red" size="1">
              {errors.email.message}
            </Text>
          )}
        </Box>

        <Box>
          <Text as="label" size="2" weight="medium" mb="1">
            パスワード
          </Text>
          <TextField.Root
            type="password"
            placeholder="8文字以上"
            {...register('password')}
          />
          {errors.password && (
            <Text color="red" size="1">
              {errors.password.message}
            </Text>
          )}
        </Box>

        <Button type="submit" size="3" disabled={isSubmitting}>
          {isSubmitting ? 'ログイン中...' : 'ログイン'}
        </Button>

        <Text size="2" align="center">
          アカウントをお持ちでない方は
          <Link asChild ml="1">
            <NextLink href="/register">新規登録</NextLink>
          </Link>
        </Text>
      </Flex>
    </form>
  );
}

export default function LoginPage() {
  return (
    <Container size="1" py="9">
      <Card size="4">
        <Heading size="6" mb="6" align="center">
          ログイン
        </Heading>
        <Suspense>
          <LoginForm />
        </Suspense>
      </Card>
    </Container>
  );
}
