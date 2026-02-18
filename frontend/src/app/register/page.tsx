'use client';

import { getMessage, Message } from '@/constants/message';
import { useAuth } from '@/contexts/AuthContext';
import { RegisterFormValues, registerSchema } from '@/schemas/auth';
import { zodResolver } from '@hookform/resolvers/zod';
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
import { AxiosError } from 'axios';
import { useRouter } from 'next/navigation';
import { useState } from 'react';
import { useForm } from 'react-hook-form';
import NextLink from 'next/link';

export default function RegisterPage() {
  const { register: registerUser } = useAuth();
  const router = useRouter();
  const [serverError, setServerError] = useState('');

  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<RegisterFormValues>({
    resolver: zodResolver(registerSchema),
  });

  const onSubmit = async (values: RegisterFormValues) => {
    setServerError('');
    try {
      await registerUser(values.email, values.password);
      router.push('/');
    } catch (error) {
      if (error instanceof AxiosError && error.response?.status === 409) {
        setServerError(getMessage(Message.E0006, 'メールアドレス'));
      } else {
        setServerError(getMessage(Message.E0005, '登録'));
      }
    }
  };

  return (
    <Container size="1" py="9">
      <Card size="4">
        <Heading size="6" mb="6" align="center">
          新規登録
        </Heading>

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

            <Box>
              <Text as="label" size="2" weight="medium" mb="1">
                パスワード（確認）
              </Text>
              <TextField.Root
                type="password"
                placeholder="パスワードを再入力"
                {...register('confirmPassword')}
              />
              {errors.confirmPassword && (
                <Text color="red" size="1">
                  {errors.confirmPassword.message}
                </Text>
              )}
            </Box>
            
            <Button type="submit" size="3" disabled={isSubmitting}>
              {isSubmitting ? '登録中...' : '新規登録'}
            </Button>
            
            <Text size='2' align='center'>
              アカウントをお持ちの方は
              <Link asChild ml='1'>
                <NextLink href="/login">ログイン</NextLink>
              </Link>
            </Text>
          </Flex>
        </form>
      </Card>
    </Container>
  );
}
