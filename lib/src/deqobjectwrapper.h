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

#ifndef DEQOBJECTWRAPPER_H
#define DEQOBJECTWRAPPER_H

#include "deqmlregister.fwd.h"
#include "deqobject.h"
#include <DOtherSide/DosQMetaObject.h>

template <int Index, int RegistrationTypeId>
class DEQObjectWrapper : public QObject, public DOS::DosIQObjectImpl
{
public:
    static const QMetaObject staticMetaObject;
    DEQObjectWrapper(QObject *parent = nullptr)
        : QObject(parent)
        , m_dObject(nullptr)
        , m_impl(nullptr)
    {
        void *impl = nullptr;
        m_data.createDObject(m_id, static_cast<QObject *>(this), &m_dObject, &impl);
        m_impl = dynamic_cast<DOS::DosIQObjectImpl *>(static_cast<QObject *>(impl));
        Q_ASSERT(m_dObject);
        Q_ASSERT(m_impl);
    }
    ~DEQObjectWrapper()
    {
        m_data.deleteDObject(m_id, m_dObject);
        m_dObject = nullptr;
        m_impl = nullptr;
    }
    const QMetaObject *metaObject() const override
    {
        Q_ASSERT(m_impl);
        return m_impl->metaObject();
    }
    int qt_metacall(QMetaObject::Call call, int index, void **args) override
    {
        Q_ASSERT(m_impl);
        return m_impl->qt_metacall(call, index, args);
    }
    bool emitSignal(QObject *, const QString &name, const std::vector<QVariant> &argumentsValues) override
    {
        Q_ASSERT(m_impl);
        return m_impl->emitSignal(this, name, argumentsValues);
    }
    static bool isRegistered()
    {
        return m_id != -1;
    }
    static int registerType(DOS::QmlRegisterType &&data)
    {
        m_data = std::move(data);
        *(const_cast<QMetaObject *>(&staticMetaObject)) = *(m_data.staticMetaObject->metaObject());

        using TypedRegistrationStrategy = DERegistrationStrategy<RegistrationTypeId>;
        m_id = TypedRegistrationStrategy::template registerType<DEQObjectWrapper>(m_data.uri.c_str(), m_data.major,
                                                                                  m_data.minor, m_data.qml.c_str());
        return m_id;
    }

private:
    void *m_dObject;
    DOS::DosIQObjectImpl *m_impl;
    static int m_id;
    static DOS::QmlRegisterType m_data;
};

template <int Index, int RegistrationTypeId>
const QMetaObject DEQObjectWrapper<Index, RegistrationTypeId>::staticMetaObject = QObject::staticMetaObject;

template <int Index, int RegistrationTypeId>
DOS::QmlRegisterType DEQObjectWrapper<Index, RegistrationTypeId>::m_data;

template <int Index, int RegistrationTypeId>
int DEQObjectWrapper<Index, RegistrationTypeId>::m_id = -1;

class DEQObjectWrapperRegisterHelper
{
public:
    template <int Index, int RegistrationTypeId>
    static int registerType(DOS::QmlRegisterType &&data)
    {
        return DEQObjectWrapper<Index, RegistrationTypeId>::registerType(std::move(data));
    }
};

#endif // DEQOBJECTWRAPPER_H
