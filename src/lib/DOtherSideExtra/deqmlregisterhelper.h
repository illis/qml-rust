/*
 * Copyright (C) 2016 Lucien XU <sfietkonstantin@free.fr>
 *
 * You may use this file under the terms of the BSD license as follows:
 *
 * "Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *   * Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in
 *     the documentation and/or other materials provided with the
 *     distribution.
 *   * The names of its contributors may not be used to endorse or promote
 *     products derived from this software without specific prior written
 *     permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE."
 */

#ifndef DEQMLREGISTERHELPER_H
#define DEQMLREGISTERHELPER_H

#include <DOtherSide/DosQMetaObject.h>

template <class TypedRegisterHelper, int RegisteryIndex, int RegistrationTypeId>
class DeQmlRegisterHelper
{
public:
    static int registerType(int i, DOS::QmlRegisterType args)
    {
        if (i > RegisteryIndex) {
            return -1;
        } else if (i == RegisteryIndex) {
            return TypedRegisterHelper::template registerType<RegisteryIndex, RegistrationTypeId>(std::move(args));
        } else {
            using NextDeQmlRegisterHelper =
                DeQmlRegisterHelper<TypedRegisterHelper, RegisteryIndex - 1, RegistrationTypeId>;
            return NextDeQmlRegisterHelper::registerType(i, std::move(args));
        }
    }
};

template <class TypedRegisterHelper, int RegistrationTypeId>
class DeQmlRegisterHelper<TypedRegisterHelper, 0, RegistrationTypeId>
{
public:
    static int registerType(int i, DOS::QmlRegisterType args)
    {
        return i == 0 ? TypedRegisterHelper::template registerType<0, RegistrationTypeId>(std::move(args)) : -1;
    }
};

#endif // DEQMLREGISTERHELPER_H
