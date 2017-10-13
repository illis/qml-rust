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

#include "deqlistmodel.h"
#include "deslotexecutor.h"
#include <DOtherSide/DosQMetaObject.h>
#include <DOtherSide/DosQObjectImpl.h>

namespace {

DOS::DosQObjectImpl::ParentMetaCall createParentMetaCall(DEQBaseListModel *parent)
{
    return [parent](QMetaObject::Call callType, int index, void **args) {
        return parent->DEQBaseListModel::qt_metacall(callType, index, args);
    };
}
}

DEQListModel::DEQListModel(DOS::DosIQMetaObjectPtr metaObject, std::map<int, QByteArray> &&roleNames,
                           DObjectCallback callback)
    : DEQBaseListModel(std::move(roleNames))
    , m_impl{new DOS::DosQObjectImpl(this, ::createParentMetaCall(this), std::move(metaObject),
                                     DESlotExecutor<DEQListModel>(*this, callback))}
{
}

bool DEQListModel::emitSignal(QObject *emitter, const QString &name, const std::vector<QVariant> &arguments)
{
    Q_ASSERT(m_impl);
    return m_impl->emitSignal(emitter, name, arguments);
}

const QMetaObject *DEQListModel::metaObject() const
{
    Q_ASSERT(m_impl);
    return m_impl->metaObject();
}

int DEQListModel::qt_metacall(QMetaObject::Call call, int index, void **args)
{
    Q_ASSERT(m_impl);
    return m_impl->qt_metacall(call, index, args);
}

void *DEQListModel::dObject() const
{
    return m_dObject;
}

void DEQListModel::setDObject(void *dObject)
{
    m_dObject = dObject;
}
